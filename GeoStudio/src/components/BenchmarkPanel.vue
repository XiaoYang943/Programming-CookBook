<script setup lang="ts">
import type { BenchmarkRecord } from '../indexers/types'

defineProps<{ records: BenchmarkRecord[] }>()

function formatMs(value: number): string {
  return value < 0.001 ? '<0.001' : value.toFixed(3)
}

function queryLabel(type: BenchmarkRecord['queryType']): string {
  if (type === 'bbox') return 'range()'
  return type === 'radius' ? 'within()' : 'withinInto()'
}
</script>

<template>
  <section class="tool-section benchmark-list">
    <div class="section-title">
      <div><span class="eyebrow">BENCHMARK</span><h2>查询记录</h2></div>
      <span class="run-count">{{ records.length }} RUNS</span>
    </div>

    <p v-if="records.length === 0" class="hint">
      执行查询后在此记录 KDBush 与 brute force 耗时。
    </p>

    <ol v-else class="benchmark-runs">
      <li v-for="record in records" :key="record.id" class="benchmark-run">
        <div class="benchmark-run__header">
          <code>{{ queryLabel(record.queryType) }}</code>
          <span
            class="consistency"
            :class="{ 'consistency--bad': !record.consistent }"
          >
            {{ record.consistent ? '一致' : '不一致' }}
          </span>
        </div>
        <dl>
          <div><dt>KDBush</dt><dd>{{ formatMs(record.kdbushMs) }} ms</dd></div>
          <div><dt>Brute force</dt><dd>{{ formatMs(record.bruteForceMs) }} ms</dd></div>
          <div><dt>命中</dt><dd>{{ record.kdbushHits }}</dd></div>
        </dl>
      </li>
    </ol>
  </section>
</template>
