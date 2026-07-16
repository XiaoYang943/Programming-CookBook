<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, watch } from 'vue'
import { isTauri } from '@tauri-apps/api/core'
import { getCurrentWebview } from '@tauri-apps/api/webview'
import type { UnlistenFn } from '@tauri-apps/api/event'
import type { GeoJSONSource } from 'maplibre-gl'
import { MaplibreTerradrawControl } from '@watergis/maplibre-gl-terradraw'
import type {
  BBox,
  KDQueryTrace,
  KDSplitLine,
  QueryType,
  RadiusQuery,
} from '../indexers/types'
import { createMap, type GeoStudioMap } from '../map/mapLibre'
import type { PointFeatureCollection } from '../utils/geojson'

interface PolygonFeatureCollection {
  type: 'FeatureCollection'
  features: Array<{
    type: 'Feature'
    properties: Record<string, unknown>
    geometry: { type: 'Polygon'; coordinates: number[][][] }
  }>
}

interface TraceRegionCollection {
  type: 'FeatureCollection'
  features: Array<{
    type: 'Feature'
    properties: { state: 'active' | 'pruned' }
    geometry: { type: 'Polygon'; coordinates: number[][][] }
  }>
}

interface LineFeatureCollection {
  type: 'FeatureCollection'
  features: Array<{
    type: 'Feature'
    properties: { axis: 'x' | 'y'; depth: number }
    geometry: { type: 'LineString'; coordinates: [number, number][] }
  }>
}

const props = defineProps<{
  dataset: PointFeatureCollection
  results: PointFeatureCollection
  queryType: QueryType
  bbox: BBox
  radiusCenter: RadiusQuery | null
  splits: KDSplitLine[]
  selectedPoints: PointFeatureCollection
  trace: KDQueryTrace | null
  traceStepIndex: number
  traceOverlayVisible: boolean
}>()

const emit = defineEmits<{
  mapClick: [coordinates: { x: number; y: number }]
  pointSelect: [pointId: string]
  bboxDrawn: [bbox: BBox]
  geoJsonDrop: [paths: string[]]
}>()

const mapContainer = ref<HTMLDivElement | null>(null)
const mapError = ref('')
const fileDragActive = ref(false)
let map: GeoStudioMap | null = null
let styleReady = false
let drawControl: MaplibreTerradrawControl | null = null
let unlistenFileDrop: UnlistenFn | null = null
let componentUnmounted = false

onMounted(() => {
  if (!mapContainer.value) return

  if (isTauri()) {
    getCurrentWebview().onDragDropEvent((event) => {
      const payload = event.payload
      if (payload.type === 'leave') {
        fileDragActive.value = false
        return
      }

      const insideMap = isPositionInsideMap(payload.position.x, payload.position.y)
      fileDragActive.value = insideMap && payload.type !== 'drop'
      if (payload.type === 'drop' && insideMap) emit('geoJsonDrop', payload.paths)
    }).then((unlisten) => {
      if (componentUnmounted) unlisten()
      else unlistenFileDrop = unlisten
    }).catch((error) => console.error('Unable to register file drop listener:', error))
  }

  try {
    map = createMap(mapContainer.value)
    drawControl = new MaplibreTerradrawControl({
      modes: ['render', 'rectangle'],
      open: false,
    })
    map.addControl(drawControl, 'top-left')
    map.on('load', () => {
      styleReady = true
      ensureExperimentLayers()
      updateSource('dataset-points', props.dataset)
      updateSource('query-results', props.results)
      mapError.value = ''
      const terraDraw = drawControl?.getTerraDrawInstance()
      terraDraw?.on('finish', handleDrawFinish)
    })
    map.on('click', (event) => {
      const features = map?.queryRenderedFeatures(event.point, {
        layers: ['dataset-points-layer', 'query-results-layer'],
      })
      const pointId = features?.[0]?.properties?.__geostudio_id
      if (pointId !== undefined && pointId !== null) emit('pointSelect', String(pointId))
      emit('mapClick', { x: event.lngLat.lng, y: event.lngLat.lat })
    })
    map.on('error', (event) => {
      console.error('MapLibre error:', event.error)
      mapError.value = '底图暂时无法加载，请检查网络连接。'
    })
  } catch (error) {
    console.error('Unable to initialize MapLibre:', error)
    mapError.value = '地图初始化失败。'
  }
})

watch(
  () => props.dataset,
  (collection) => {
    updateSource('dataset-points', collection)
    if (!map || !styleReady || collection.features.length === 0) return

    let minX = Infinity
    let minY = Infinity
    let maxX = -Infinity
    let maxY = -Infinity
    for (const feature of collection.features) {
      const [x, y] = feature.geometry.coordinates
      minX = Math.min(minX, x)
      minY = Math.min(minY, y)
      maxX = Math.max(maxX, x)
      maxY = Math.max(maxY, y)
    }
    map.fitBounds(
      [
        [minX, minY],
        [maxX, maxY],
      ],
      { padding: 64, maxZoom: 14, duration: 650 },
    )
  },
)

watch(
  () => [props.trace, props.traceStepIndex, props.traceOverlayVisible] as const,
  () => updateTraceVisualization(),
  { deep: true },
)

watch(
  () => props.results,
  (collection) => updateSource('query-results', collection),
)

watch(
  () => props.selectedPoints,
  (collection) => updateSource('selected-index-point', collection),
)

watch(
  () => props.splits,
  () => updateSplitLines(),
  { deep: true },
)

watch(
  () => [props.queryType, props.bbox, props.radiusCenter] as const,
  () => updateQueryArea(),
  { deep: true },
)

onBeforeUnmount(() => {
  componentUnmounted = true
  unlistenFileDrop?.()
  unlistenFileDrop = null
  const terraDraw = drawControl?.getTerraDrawInstance()
  terraDraw?.off('finish', handleDrawFinish)
  drawControl?.deactivate()
  drawControl = null
  map?.remove()
  map = null
  styleReady = false
})

function isPositionInsideMap(physicalX: number, physicalY: number): boolean {
  const element = mapContainer.value
  if (!element) return false
  const scale = window.devicePixelRatio || 1
  const x = physicalX / scale
  const y = physicalY / scale
  const bounds = element.getBoundingClientRect()
  return x >= bounds.left && x <= bounds.right && y >= bounds.top && y <= bounds.bottom
}

function ensureExperimentLayers(): void {
  if (!map || map.getSource('dataset-points')) return

  map.addSource('dataset-points', { type: 'geojson', data: props.dataset })
  map.addLayer({
    id: 'dataset-points-layer',
    type: 'circle',
    source: 'dataset-points',
    paint: {
      'circle-radius': ['interpolate', ['linear'], ['zoom'], 2, 2.5, 12, 5],
      'circle-color': '#38bdf8',
      'circle-opacity': 0.68,
      'circle-stroke-color': '#e0f2fe',
      'circle-stroke-width': 0.5,
    },
  })

  map.addSource('query-results', { type: 'geojson', data: props.results })
  map.addLayer({
    id: 'query-results-layer',
    type: 'circle',
    source: 'query-results',
    paint: {
      'circle-radius': ['interpolate', ['linear'], ['zoom'], 2, 5, 12, 9],
      'circle-color': '#fb7185',
      'circle-opacity': 0.95,
      'circle-stroke-color': '#fff1f2',
      'circle-stroke-width': 1.5,
    },
  })

  map.addSource('selected-index-point', { type: 'geojson', data: props.selectedPoints })
  map.addLayer({
    id: 'selected-index-point-layer',
    type: 'circle',
    source: 'selected-index-point',
    paint: {
      'circle-radius': 11,
      'circle-color': '#f8fafc',
      'circle-opacity': 0.96,
      'circle-stroke-color': '#f59e0b',
      'circle-stroke-width': 3,
    },
  })

  map.addSource('query-area', { type: 'geojson', data: createQueryArea() })
  map.addLayer({
    id: 'query-area-fill',
    type: 'fill',
    source: 'query-area',
    paint: { 'fill-color': '#fbbf24', 'fill-opacity': 0.08 },
  })
  map.addLayer({
    id: 'query-area-line',
    type: 'line',
    source: 'query-area',
    paint: { 'line-color': '#fbbf24', 'line-width': 2, 'line-dasharray': [3, 2] },
  })

  map.addSource('kd-splits', { type: 'geojson', data: createSplitLines() })
  map.addLayer({
    id: 'kd-splits-x',
    type: 'line',
    source: 'kd-splits',
    filter: ['==', ['get', 'axis'], 'x'],
    paint: { 'line-color': '#38bdf8', 'line-width': 1.5, 'line-opacity': 0.8 },
  })
  map.addLayer({
    id: 'kd-splits-y',
    type: 'line',
    source: 'kd-splits',
    filter: ['==', ['get', 'axis'], 'y'],
    paint: { 'line-color': '#34d399', 'line-width': 1.5, 'line-opacity': 0.8 },
  })

  map.addSource('trace-regions', { type: 'geojson', data: createTraceRegions() })
  map.addLayer({
    id: 'trace-pruned-regions',
    type: 'fill',
    source: 'trace-regions',
    filter: ['==', ['get', 'state'], 'pruned'],
    paint: { 'fill-color': '#64748b', 'fill-opacity': 0.22 },
  })
  map.addLayer({
    id: 'trace-active-region',
    type: 'fill',
    source: 'trace-regions',
    filter: ['==', ['get', 'state'], 'active'],
    paint: { 'fill-color': '#fbbf24', 'fill-opacity': 0.2 },
  })
  map.addSource('trace-point', { type: 'geojson', data: createTracePoint() })
  map.addLayer({
    id: 'trace-point-layer',
    type: 'circle',
    source: 'trace-point',
    paint: {
      'circle-radius': 9,
      'circle-color': '#fbbf24',
      'circle-stroke-color': '#422006',
      'circle-stroke-width': 2,
    },
  })
  map.moveLayer('query-results-layer')
  map.moveLayer('selected-index-point-layer')
  map.moveLayer('trace-point-layer')
}

function updateSource(sourceId: string, collection: PointFeatureCollection): void {
  if (!map || !styleReady) return
  const source = map.getSource(sourceId) as GeoJSONSource | undefined
  source?.setData(collection)
}

function getCurrentBounds(): BBox | null {
  if (!map) return null
  const bounds = map.getBounds()
  return {
    minX: bounds.getWest(),
    minY: bounds.getSouth(),
    maxX: bounds.getEast(),
    maxY: bounds.getNorth(),
  }
}

function startBBoxDrawing(): void {
  const terraDraw = drawControl?.getTerraDrawInstance()
  if (!terraDraw) return
  terraDraw.clear()
  terraDraw.setMode('rectangle')
}

function handleDrawFinish(featureId: string | number): void {
  const terraDraw = drawControl?.getTerraDrawInstance()
  if (!terraDraw) return
  const feature = terraDraw.getSnapshotFeature(featureId)
  if (!feature || feature.geometry.type !== 'Polygon') return
  const ring = feature.geometry.coordinates[0]
  if (!ring?.length) return

  let minX = Infinity
  let minY = Infinity
  let maxX = -Infinity
  let maxY = -Infinity
  for (const coordinate of ring) {
    minX = Math.min(minX, coordinate[0])
    minY = Math.min(minY, coordinate[1])
    maxX = Math.max(maxX, coordinate[0])
    maxY = Math.max(maxY, coordinate[1])
  }
  if (![minX, minY, maxX, maxY].every(Number.isFinite)) return

  emit('bboxDrawn', { minX, minY, maxX, maxY })
  terraDraw.clear()
  terraDraw.setMode('render')
}

function updateQueryArea(): void {
  if (!map || !styleReady) return
  const source = map.getSource('query-area') as GeoJSONSource | undefined
  source?.setData(createQueryArea())
}

function updateSplitLines(): void {
  if (!map || !styleReady) return
  const source = map.getSource('kd-splits') as GeoJSONSource | undefined
  source?.setData(createSplitLines())
}

function createSplitLines(): LineFeatureCollection {
  return {
    type: 'FeatureCollection',
    features: props.splits.map((split) => ({
      type: 'Feature',
      properties: { axis: split.axis, depth: split.depth },
      geometry: { type: 'LineString', coordinates: [split.start, split.end] },
    })),
  }
}

function updateTraceVisualization(): void {
  if (!map || !styleReady) return
  const regions = map.getSource('trace-regions') as GeoJSONSource | undefined
  const point = map.getSource('trace-point') as GeoJSONSource | undefined
  regions?.setData(createTraceRegions())
  point?.setData(createTracePoint())
}

function createTraceRegions(): TraceRegionCollection {
  if (!props.traceOverlayVisible || !props.trace?.steps.length) {
    return { type: 'FeatureCollection', features: [] }
  }
  const visibleSteps = props.trace.steps.slice(0, props.traceStepIndex + 1)
  const currentStep = visibleSteps[visibleSteps.length - 1]
  const features: TraceRegionCollection['features'] = visibleSteps
    .filter((step) => step.kind === 'prune')
    .map((step) => regionFeature(step.region, 'pruned'))
  if (currentStep && currentStep.kind !== 'complete') {
    features.push(regionFeature(currentStep.region, 'active'))
  }
  return { type: 'FeatureCollection', features }
}

function regionFeature(
  bounds: BBox,
  state: 'active' | 'pruned',
): TraceRegionCollection['features'][number] {
  return {
    type: 'Feature',
    properties: { state },
    geometry: {
      type: 'Polygon',
      coordinates: [[
        [bounds.minX, bounds.minY],
        [bounds.maxX, bounds.minY],
        [bounds.maxX, bounds.maxY],
        [bounds.minX, bounds.maxY],
        [bounds.minX, bounds.minY],
      ]],
    },
  }
}

function createTracePoint(): PointFeatureCollection {
  if (!props.traceOverlayVisible) return { type: 'FeatureCollection', features: [] }
  const pointId = props.trace?.steps[props.traceStepIndex]?.pointId
  if (!pointId) return { type: 'FeatureCollection', features: [] }
  return {
    type: 'FeatureCollection',
    features: props.dataset.features.filter(
      (feature) => String(feature.properties?.__geostudio_id) === pointId,
    ),
  }
}

function createQueryArea(): PolygonFeatureCollection {
  const coordinates =
    props.queryType === 'bbox'
      ? [
          [props.bbox.minX, props.bbox.minY],
          [props.bbox.maxX, props.bbox.minY],
          [props.bbox.maxX, props.bbox.maxY],
          [props.bbox.minX, props.bbox.maxY],
          [props.bbox.minX, props.bbox.minY],
        ]
      : createRadiusRing(props.radiusCenter)

  return {
    type: 'FeatureCollection',
    features: coordinates.length
      ? [{ type: 'Feature', properties: {}, geometry: { type: 'Polygon', coordinates: [coordinates] } }]
      : [],
  }
}

function createRadiusRing(query: RadiusQuery | null): number[][] {
  if (!query || !Number.isFinite(query.radius) || query.radius < 0) return []
  const ring: number[][] = []
  for (let index = 0; index <= 64; index += 1) {
    const angle = (index / 64) * Math.PI * 2
    ring.push([
      query.x + Math.cos(angle) * query.radius,
      query.y + Math.sin(angle) * query.radius,
    ])
  }
  return ring
}

defineExpose({ getCurrentBounds, startBBoxDrawing })
</script>

<template>
  <section class="map-workspace" aria-label="GeoStudio 地图工作区">
    <div ref="mapContainer" class="map-container" />
    <div v-if="fileDragActive" class="map-drop-overlay">
      <div class="map-drop-overlay__content">
        <strong>释放以加载 GeoJSON</strong>
        <span>支持单个 .geojson 或 .json 点数据文件</span>
      </div>
    </div>
    <div class="map-legend">
      <div class="map-legend__items">
        <span><i class="legend-dot legend-dot--data" />原始点</span>
        <span><i class="legend-dot legend-dot--result" />查询命中</span>
        <template v-if="splits.length">
          <span><i class="legend-line legend-line--x" />X 轴分割</span>
          <span><i class="legend-line legend-line--y" />Y 轴分割</span>
        </template>
        <template v-if="trace?.steps.length">
          <span><i class="legend-area legend-area--active" />当前处理区域</span>
          <span><i class="legend-area legend-area--pruned" />已剪枝区域</span>
          <span><i class="legend-dot legend-dot--checking" />当前检查点</span>
        </template>
      </div>
      <small v-if="trace?.steps.length" class="map-legend__help">
        右侧播放区显示当前数组范围、分割轴和本步判断结果。
      </small>
      <span v-if="radiusCenter" class="map-coordinate">
        圆心 {{ radiusCenter.x.toFixed(4) }}, {{ radiusCenter.y.toFixed(4) }}
      </span>
    </div>
    <p v-if="mapError" class="map-error" role="status">{{ mapError }}</p>
  </section>
</template>
