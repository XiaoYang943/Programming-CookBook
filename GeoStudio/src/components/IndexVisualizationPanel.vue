<script setup lang="ts">
import { computed, onBeforeUnmount, ref } from 'vue'
import type {
  KDIndexEntry,
  KDQueryTrace,
  KDStructureSnapshot,
} from '../indexers/types'

const props = defineProps<{
  snapshot: KDStructureSnapshot | null
  trace: KDQueryTrace | null
  depth: number
  selectedPosition: number | null
  traceStepIndex: number
}>()

const emit = defineEmits<{
  'update:depth': [value: number]
  select: [entry: KDIndexEntry]
  'update:traceStepIndex': [value: number]
  clearTrace: []
}>()

const playing = ref(false)
let playbackTimer: ReturnType<typeof setInterval> | null = null
const currentStep = computed(() => props.trace?.steps[props.traceStepIndex] ?? null)
const activePosition = computed(() => currentStep.value?.pointPosition ?? props.selectedPosition)
const treePath = computed(() => {
  if (!props.snapshot || activePosition.value === null) return []
  return props.snapshot.nodes
    .filter((node) => activePosition.value! >= node.left && activePosition.value! <= node.right)
    .sort((left, right) => left.depth - right.depth)
})
const accumulatedHitCount = computed(() => {
  if (!props.trace) return 0
  return props.trace.steps
    .slice(0, props.traceStepIndex + 1)
    .reduce((count, step) => count + step.newlyMatchedIds.length, 0)
})

function updateDepth(event: Event): void {
  emit('update:depth', Number((event.target as HTMLInputElement).value))
}

function setStep(index: number): void {
  if (!props.trace?.steps.length) return
  const next = Math.min(Math.max(index, 0), props.trace.steps.length - 1)
  emit('update:traceStepIndex', next)
}

function togglePlayback(): void {
  if (playing.value) {
    stopPlayback()
    return
  }
  if (!props.trace?.steps.length) return
  if (props.traceStepIndex >= props.trace.steps.length - 1) {
    setStep(0)
  }
  playing.value = true
  playbackTimer = setInterval(() => {
    if (!props.trace || props.traceStepIndex >= props.trace.steps.length - 1) {
      stopPlayback()
      return
    }
    setStep(props.traceStepIndex + 1)
  }, 650)
}

function showFinalResult(): void {
  stopPlayback()
  if (!props.trace?.steps.length) return
  setStep(props.trace.steps.length - 1)
}

function stopPlayback(): void {
  playing.value = false
  if (playbackTimer) clearInterval(playbackTimer)
  playbackTimer = null
}

function selectTreeNode(position: number): void {
  const entry = props.snapshot?.entries[position]
  if (entry) emit('select', entry)
}

onBeforeUnmount(stopPlayback)
</script>

<template>
  <section class="tool-section index-visualization">
    <div class="section-title">
      <div><span class="eyebrow">STRUCTURE</span><h2>索引可视化</h2></div>
      <span v-if="snapshot" class="split-count">{{ snapshot.splits.length }} SPLITS</span>
    </div>

    <template v-if="snapshot">
      <label class="field">
        <span>空间分割深度：{{ depth }}</span>
        <input
          :value="depth"
          type="range"
          min="0"
          max="8"
          step="1"
          @input="updateDepth"
        />
      </label>
      <p class="hint">蓝色竖线表示 X 轴分割，绿色横线表示 Y 轴分割。分割线由扁平数组推导，不存储在线索引中。</p>

      <div v-if="trace?.steps.length" class="playback">
        <div class="playback__controls">
          <button class="button button--ghost" @click="setStep(0)">起点</button>
          <button class="button button--ghost" @click="setStep(traceStepIndex - 1)">上一步</button>
          <button class="button button--primary" @click="togglePlayback">{{ playing ? '暂停' : '播放' }}</button>
          <button class="button button--ghost" @click="setStep(traceStepIndex + 1)">下一步</button>
          <button class="button button--ghost" @click="showFinalResult">结果</button>
          <button class="button button--ghost" @click="emit('clearTrace')">清除</button>
        </div>
        <input
          :value="traceStepIndex"
          type="range"
          min="0"
          :max="trace.steps.length - 1"
          step="1"
          @input="setStep(Number(($event.target as HTMLInputElement).value))"
        />
        <p class="playback__step">{{ traceStepIndex + 1 }} / {{ trace.steps.length }} · {{ currentStep?.description }}</p>
      </div>
      <div v-else class="playback-placeholder">
        <strong>等待 BBox 查询</strong>
        <p>选择 BBox / range() 并执行对照查询后，即可逐步播放 KD-tree 的访问与剪枝过程。</p>
      </div>

      <dl v-if="currentStep" class="current-step-metrics">
        <div><dt>当前区间</dt><dd>[{{ currentStep.range[0] }}, {{ currentStep.range[1] }}]</dd></div>
        <div><dt>分割轴</dt><dd>{{ currentStep.axis.toUpperCase() }}</dd></div>
        <div><dt>检查点</dt><dd>{{ currentStep.pointId ?? '—' }}</dd></div>
        <div><dt>累计命中</dt><dd>{{ accumulatedHitCount }}</dd></div>
      </dl>

      <dl v-if="trace" class="trace-metrics">
        <div><dt>访问区间</dt><dd>{{ trace.visitedNodes }}</dd></div>
        <div><dt>剪枝区间</dt><dd>{{ trace.prunedNodes }}</dd></div>
        <div><dt>检查点数</dt><dd>{{ trace.checkedPoints }} / {{ trace.totalPoints }}</dd></div>
      </dl>

      <div v-if="treePath.length" class="tree-path" aria-label="当前点的隐式树路径">
        <span class="tree-path__label">隐式树路径</span>
        <button
          v-for="node in treePath"
          :key="`${node.depth}-${node.position}`"
          class="tree-node"
          :class="{ 'tree-node--current': node.position === activePosition }"
          @click="selectTreeNode(node.position)"
        >
          D{{ node.depth }} · {{ node.axis.toUpperCase() }} · [{{ node.left }}…{{ node.right }}]
        </button>
      </div>
    </template>
    <p v-else class="hint">构建 KDBush 索引后显示空间分割和扁平数组。</p>
  </section>
</template>
