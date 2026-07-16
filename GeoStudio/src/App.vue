<script setup lang="ts">
import { computed, ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { readTextFile } from '@tauri-apps/plugin-fs'
import BenchmarkPanel from './components/BenchmarkPanel.vue'
import IndexVisualizationPanel from './components/IndexVisualizationPanel.vue'
import MapView from './components/MapView.vue'
import { BruteForceIndexer } from './indexers/bruteForceIndexer'
import { KDBushIndexer } from './indexers/kdbushIndexer'
import type {
  BBox,
  BenchmarkRecord,
  KDIndexEntry,
  KDQueryTrace,
  QueryResult,
  QueryType,
  RadiusQuery,
} from './indexers/types'
import {
  parsePointGeoJSON,
  selectFeaturesByIds,
  type ParsedPointDataset,
  type PointFeatureCollection,
} from './utils/geojson'

interface MapViewApi {
  getCurrentBounds: () => BBox | null
  startBBoxDrawing: () => void
}

const EMPTY_COLLECTION: PointFeatureCollection = {
  type: 'FeatureCollection',
  features: [],
}

const mapView = ref<MapViewApi | null>(null)
const dataset = ref<ParsedPointDataset | null>(null)
const datasetName = ref('')
const datasetPath = ref('')
const resultCollection = ref<PointFeatureCollection>(EMPTY_COLLECTION)
const busy = ref(false)
const statusMessage = ref('请选择一个 GeoJSON 点数据文件。')
const statusKind = ref<'info' | 'success' | 'error'>('info')
const indexReady = ref(false)
const buildElapsedMs = ref<number | null>(null)
const benchmarkRecords = ref<BenchmarkRecord[]>([])
const queryType = ref<QueryType>('bbox')
const bbox = ref<BBox>({ minX: 70, minY: 15, maxX: 140, maxY: 55 })
const radiusQuery = ref<RadiusQuery>({ x: 104.1954, y: 35.8617, radius: 1 })
const lastHitCount = ref<number | null>(null)
const visualizationDepth = ref(4)
const selectedIndexEntry = ref<KDIndexEntry | null>(null)
const queryTrace = ref<KDQueryTrace | null>(null)
const traceStepIndex = ref(0)
const traceOverlayVisible = ref(true)

const kdbush = new KDBushIndexer()
const bruteForce = new BruteForceIndexer()

const indexInfo = computed(() => kdbush.getInfo())
const canQuery = computed(() => indexReady.value && !busy.value)
const structureSnapshot = computed(() => {
  if (!indexReady.value || !dataset.value?.bounds) return null
  return kdbush.getStructureSnapshot(
    {
      minX: dataset.value.bounds[0],
      minY: dataset.value.bounds[1],
      maxX: dataset.value.bounds[2],
      maxY: dataset.value.bounds[3],
    },
    visualizationDepth.value,
  )
})
const selectedPointCollection = computed<PointFeatureCollection>(() => {
  if (!dataset.value || !selectedIndexEntry.value) return EMPTY_COLLECTION
  return selectFeaturesByIds(dataset.value.featureCollection, [selectedIndexEntry.value.pointId])
})
const currentTraceStep = computed(() => queryTrace.value?.steps[traceStepIndex.value] ?? null)
const visibleResultCollection = computed<PointFeatureCollection>(() => {
  if (!dataset.value || !queryTrace.value?.steps.length) return resultCollection.value
  const visibleIds = queryTrace.value.steps
    .slice(0, traceStepIndex.value + 1)
    .flatMap((step) => step.newlyMatchedIds)
  return selectFeaturesByIds(dataset.value.featureCollection, visibleIds)
})

async function chooseGeoJSON(): Promise<void> {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'GeoJSON', extensions: ['geojson', 'json'] }],
    })
    if (!selected || Array.isArray(selected)) return
    await loadGeoJSON(selected)
  } catch (error) {
    setStatus(toErrorMessage(error), 'error')
  }
}

async function handleGeoJSONDrop(paths: string[]): Promise<void> {
  if (busy.value) return
  if (paths.length !== 1) {
    setStatus('目前仅支持一次拖入一个 GeoJSON 文件。', 'error')
    return
  }

  const path = paths[0]
  if (!/\.(?:geojson|json)$/i.test(path)) {
    setStatus('请拖入 .geojson 或 .json 文件。', 'error')
    return
  }

  await loadGeoJSON(path)
}

async function loadGeoJSON(path: string): Promise<void> {
  try {
    busy.value = true
    setStatus('正在读取并解析本地 GeoJSON…', 'info')
    const text = await readTextFile(path)
    const parsed = parsePointGeoJSON(text)

    dataset.value = parsed
    datasetPath.value = path
    datasetName.value = path.split(/[\\/]/).pop() || '未命名数据集'
    resultCollection.value = EMPTY_COLLECTION
    indexReady.value = false
    buildElapsedMs.value = null
    lastHitCount.value = null
    selectedIndexEntry.value = null
    queryTrace.value = null
    traceStepIndex.value = 0
    traceOverlayVisible.value = true
    kdbush.clear()
    bruteForce.build(parsed.points)

    if (parsed.bounds) {
      bbox.value = {
        minX: parsed.bounds[0],
        minY: parsed.bounds[1],
        maxX: parsed.bounds[2],
        maxY: parsed.bounds[3],
      }
      radiusQuery.value.x = (parsed.bounds[0] + parsed.bounds[2]) / 2
      radiusQuery.value.y = (parsed.bounds[1] + parsed.bounds[3]) / 2
    }

    const skipped = parsed.invalidFeatureCount
      ? `，跳过 ${parsed.invalidFeatureCount} 个无效或非点 Feature`
      : ''
    setStatus(`已加载 ${parsed.points.length} 个有效点${skipped}。`, 'success')
  } catch (error) {
    setStatus(toErrorMessage(error), 'error')
  } finally {
    busy.value = false
  }
}

function buildIndex(): void {
  if (!dataset.value) {
    setStatus('请先选择 GeoJSON 点数据。', 'error')
    return
  }

  try {
    busy.value = true
    setStatus('正在构建 KDBush 静态索引…', 'info')
    const startedAt = performance.now()
    kdbush.build(dataset.value.points)
    buildElapsedMs.value = performance.now() - startedAt
    indexReady.value = true
    selectedIndexEntry.value = null
    queryTrace.value = null
    traceStepIndex.value = 0
    traceOverlayVisible.value = true
    setStatus(`KDBush 索引构建完成，用时 ${formatMs(buildElapsedMs.value)}。`, 'success')
  } catch (error) {
    indexReady.value = false
    setStatus(toErrorMessage(error), 'error')
  } finally {
    busy.value = false
  }
}

function useCurrentMapBounds(): void {
  const currentBounds = mapView.value?.getCurrentBounds()
  if (!currentBounds) {
    setStatus('地图尚未准备好。', 'error')
    return
  }
  bbox.value = currentBounds
  setStatus('已将当前地图视野写入 bbox 参数。', 'info')
}

function startBBoxDrawing(): void {
  if (!dataset.value) {
    setStatus('请先加载 GeoJSON 点数据。', 'error')
    return
  }
  queryType.value = 'bbox'
  mapView.value?.startBBoxDrawing()
  setStatus('请在地图上按住鼠标拖动绘制 BBox。', 'info')
}

function applyDrawnBBox(value: BBox): void {
  bbox.value = value
  queryType.value = 'bbox'
  setStatus('BBox 绘制完成，参数已更新；确认后可执行查询。', 'success')
}

function resetBBoxToDataset(): void {
  if (!dataset.value?.bounds) return
  bbox.value = {
    minX: dataset.value.bounds[0],
    minY: dataset.value.bounds[1],
    maxX: dataset.value.bounds[2],
    maxY: dataset.value.bounds[3],
  }
  queryType.value = 'bbox'
  setStatus('BBox 已重置为输入点数据的整体范围。', 'info')
}

function handleMapClick(coordinates: { x: number; y: number }): void {
  radiusQuery.value.x = coordinates.x
  radiusQuery.value.y = coordinates.y
  if (queryType.value !== 'bbox') {
    setStatus('已从地图选择 radius 查询圆心。', 'info')
  }
}

function runQuery(): void {
  if (!dataset.value || !canQuery.value) {
    setStatus('请先加载数据并构建 KDBush 索引。', 'error')
    return
  }

  try {
    busy.value = true
    let indexedResult: QueryResult
    let bruteResult: QueryResult
    let parameters: string

    if (queryType.value === 'bbox') {
      validateBBox(bbox.value)
      indexedResult = kdbush.range(bbox.value)
      bruteResult = bruteForce.range(bbox.value)
      parameters = formatBBox(bbox.value)
      if (dataset.value.bounds) {
        queryTrace.value = kdbush.traceRange(bbox.value, {
          minX: dataset.value.bounds[0],
          minY: dataset.value.bounds[1],
          maxX: dataset.value.bounds[2],
          maxY: dataset.value.bounds[3],
        })
        traceStepIndex.value = 0
        traceOverlayVisible.value = true
      }
    } else {
      validateRadius(radiusQuery.value)
      indexedResult =
        queryType.value === 'radius-into'
          ? kdbush.withinInto(radiusQuery.value)
          : kdbush.within(radiusQuery.value)
      bruteResult = bruteForce.within(radiusQuery.value)
      parameters = formatRadius(radiusQuery.value)
      queryTrace.value = null
      traceStepIndex.value = 0
      traceOverlayVisible.value = true
    }

    const officialConsistent = haveSameIds(indexedResult.ids, bruteResult.ids)
    const traceConsistent = queryTrace.value
      ? haveSameIds(indexedResult.ids, queryTrace.value.matchedIds)
      : true
    const consistent = officialConsistent && traceConsistent
    resultCollection.value = selectFeaturesByIds(
      dataset.value.featureCollection,
      indexedResult.ids,
    )
    lastHitCount.value = indexedResult.hitCount
    benchmarkRecords.value.unshift({
      id: Date.now(),
      datasetName: datasetName.value,
      pointCount: dataset.value.points.length,
      queryType: queryType.value,
      parameters,
      kdbushMs: indexedResult.elapsedMs,
      bruteForceMs: bruteResult.elapsedMs,
      kdbushHits: indexedResult.hitCount,
      bruteForceHits: bruteResult.hitCount,
      consistent,
      createdAt: new Date().toLocaleTimeString(),
    })
    setStatus(
      `查询完成：命中 ${indexedResult.hitCount} 个点，KDBush、教学追踪与 brute force 结果${consistent ? '一致' : '不一致'}。`,
      consistent ? 'success' : 'error',
    )
  } catch (error) {
    setStatus(toErrorMessage(error), 'error')
  } finally {
    busy.value = false
  }
}

function clearResults(): void {
  resultCollection.value = EMPTY_COLLECTION
  lastHitCount.value = null
  setStatus('已清除地图查询结果。', 'info')
}

function selectPointById(pointId: string): void {
  const entry = structureSnapshot.value?.entries.find((candidate) => candidate.pointId === pointId)
  if (entry) selectedIndexEntry.value = entry
}

function updateTraceStepIndex(value: number): void {
  traceStepIndex.value = value
  traceOverlayVisible.value = true
}

function clearTraceOverlay(): void {
  traceOverlayVisible.value = false
  setStatus('已清除查询播放痕迹，最终命中结果仍保留。', 'info')
}

function setStatus(message: string, kind: 'info' | 'success' | 'error'): void {
  statusMessage.value = message
  statusKind.value = kind
}

function validateBBox(value: BBox): void {
  const coordinates = [value.minX, value.minY, value.maxX, value.maxY]
  if (!coordinates.every(Number.isFinite)) throw new Error('bbox 参数必须是有效数字。')
  if (value.minX > value.maxX || value.minY > value.maxY) {
    throw new Error('bbox 最小值不能大于最大值。')
  }
}

function validateRadius(value: RadiusQuery): void {
  if (![value.x, value.y, value.radius].every(Number.isFinite)) {
    throw new Error('圆心和半径必须是有效数字。')
  }
  if (value.radius < 0) throw new Error('半径不能小于 0。')
}

function haveSameIds(left: string[], right: string[]): boolean {
  if (left.length !== right.length) return false
  const rightIds = new Set(right)
  return left.every((id) => rightIds.has(id))
}

function formatBBox(value: BBox): string {
  return `${value.minX.toFixed(4)}, ${value.minY.toFixed(4)}, ${value.maxX.toFixed(4)}, ${value.maxY.toFixed(4)}`
}

function formatRadius(value: RadiusQuery): string {
  return `${value.x.toFixed(4)}, ${value.y.toFixed(4)}, r=${value.radius}`
}

function formatMs(value: number): string {
  return `${value.toFixed(3)} ms`
}

function formatBytes(value: number): string {
  if (value < 1024) return `${value} B`
  return `${(value / 1024).toFixed(2)} KiB`
}

function toErrorMessage(error: unknown): string {
  return error instanceof Error ? error.message : String(error)
}
</script>

<template>
  <main class="gis-shell">
    <header class="topbar">
      <div class="brand">
        <span class="brand__mark" />
        <div>
          <strong>GeoStudio</strong>
        </div>
      </div>
      <div class="status" :class="`status--${statusKind}`">
        <span class="status__dot" />
        {{ statusMessage }}
      </div>
    </header>

    <section class="workspace">
      <aside class="panel panel--left">
        <div class="panel__header">
          <div>
            <span class="eyebrow">DATA</span>
            <h2>本地数据</h2>
          </div>
          <button class="button button--primary" :disabled="busy" @click="chooseGeoJSON">
            打开 GeoJSON
          </button>
        </div>

        <div v-if="dataset" class="dataset-card">
          <div class="dataset-card__icon">GJ</div>
          <div class="dataset-card__body">
            <strong :title="datasetPath">{{ datasetName }}</strong>
            <span>WGS84 · Point</span>
          </div>
          <span class="badge">{{ dataset.points.length }}</span>
        </div>
        <div v-else class="empty-state">
          <span class="empty-state__symbol">＋</span>
          <strong>尚未加载数据</strong>
          <p>选择本地 FeatureCollection&lt;Point&gt; 文件开始实验。</p>
        </div>

        <dl v-if="dataset" class="metrics">
          <div><dt>Feature 总数</dt><dd>{{ dataset.totalFeatureCount }}</dd></div>
          <div><dt>有效点</dt><dd>{{ dataset.points.length }}</dd></div>
          <div><dt>跳过</dt><dd>{{ dataset.invalidFeatureCount }}</dd></div>
          <div><dt>高亮结果</dt><dd>{{ lastHitCount ?? '—' }}</dd></div>
        </dl>

        <div class="notice">
          <strong>输入边界</strong>
          <p>首版仅支持 WGS84 GeoJSON 点数据。文件在本地读取，不上传网络。</p>
        </div>
      </aside>

      <section class="map-stage">
        <MapView
          ref="mapView"
          :dataset="dataset?.featureCollection ?? EMPTY_COLLECTION"
          :results="visibleResultCollection"
          :query-type="queryType"
          :bbox="bbox"
          :radius-center="queryType === 'bbox' ? null : radiusQuery"
          :splits="structureSnapshot?.splits ?? []"
          :selected-points="selectedPointCollection"
          :trace="queryTrace"
          :trace-step-index="traceStepIndex"
          :trace-overlay-visible="traceOverlayVisible"
          @map-click="handleMapClick"
          @point-select="selectPointById"
          @geo-json-drop="handleGeoJSONDrop"
          @bbox-drawn="applyDrawnBBox"
        />
      </section>

      <aside class="panel panel--right">
        <section class="tool-section">
          <div class="section-title">
            <div><span class="eyebrow">INDEX</span><h2>KDBush 索引</h2></div>
            <span class="index-state" :class="{ 'index-state--ready': indexReady }">
              {{ indexReady ? 'READY' : 'EMPTY' }}
            </span>
          </div>
          <button
            class="button button--primary button--wide"
            :disabled="!dataset || busy"
            @click="buildIndex"
          >
            构建静态索引
          </button>
          <dl class="metrics metrics--compact">
            <div><dt>构建耗时</dt><dd>{{ buildElapsedMs === null ? '—' : formatMs(buildElapsedMs) }}</dd></div>
            <div><dt>索引点数</dt><dd>{{ indexInfo?.numItems ?? '—' }}</dd></div>
            <div><dt>索引大小</dt><dd>{{ indexInfo ? formatBytes(indexInfo.byteLength) : '—' }}</dd></div>
            <div><dt>nodeSize</dt><dd>{{ indexInfo?.nodeSize ?? '—' }}</dd></div>
          </dl>
        </section>

        <section class="tool-section tool-section--query">
          <div class="section-title"><div><span class="eyebrow">QUERY</span><h2>空间查询</h2></div></div>
          <label class="field">
            <span>查询类型</span>
            <select v-model="queryType">
              <option value="bbox">BBox / range()</option>
              <option value="radius">Radius / within()</option>
              <option value="radius-into">Radius / withinInto()</option>
            </select>
          </label>

          <template v-if="queryType === 'bbox'">
            <div class="coordinate-grid">
              <label><span>minX</span><input v-model.number="bbox.minX" type="number" step="any" /></label>
              <label><span>minY</span><input v-model.number="bbox.minY" type="number" step="any" /></label>
              <label><span>maxX</span><input v-model.number="bbox.maxX" type="number" step="any" /></label>
              <label><span>maxY</span><input v-model.number="bbox.maxY" type="number" step="any" /></label>
            </div>
            <div class="bbox-actions">
              <button class="button button--primary" @click="startBBoxDrawing">绘制 BBox</button>
              <button class="button button--ghost" @click="useCurrentMapBounds">当前视野</button>
              <button class="button button--ghost" @click="resetBBoxToDataset">数据范围</button>
            </div>
          </template>

          <template v-else>
            <div class="coordinate-grid">
              <label><span>中心 X</span><input v-model.number="radiusQuery.x" type="number" step="any" /></label>
              <label><span>中心 Y</span><input v-model.number="radiusQuery.y" type="number" step="any" /></label>
            </div>
            <label class="field">
              <span>半径（坐标单位：度）</span>
              <input v-model.number="radiusQuery.radius" type="number" min="0" step="0.1" />
            </label>
            <p class="hint">点击地图可选择圆心。基础查询是二维欧氏距离，不代表真实米或千米。</p>
          </template>

          <div class="button-row">
            <button class="button button--primary" :disabled="!canQuery" @click="runQuery">执行对照查询</button>
            <button class="button button--ghost" :disabled="!lastHitCount" @click="clearResults">清除结果</button>
          </div>
        </section>

        <IndexVisualizationPanel
          v-model:depth="visualizationDepth"
          :snapshot="structureSnapshot"
          :trace="queryTrace"
          :trace-step-index="traceStepIndex"
          :selected-position="selectedIndexEntry?.position ?? null"
          @select="selectedIndexEntry = $event"
          @update:trace-step-index="updateTraceStepIndex"
          @clear-trace="clearTraceOverlay"
        />
        <BenchmarkPanel :records="benchmarkRecords" />
      </aside>
    </section>
  </main>
</template>
