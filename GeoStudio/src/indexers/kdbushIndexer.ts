import KDBush from 'kdbush'
import type {
  BBox,
  KDQueryTrace,
  KDStructureSnapshot,
  KDSplitLine,
  KDTreeNode,
  PointSpatialIndexer,
  QueryResult,
  RadiusQuery,
  SpatialPoint,
} from './types'

export class KDBushIndexer implements PointSpatialIndexer {
  readonly name = 'KDBush'

  private index: KDBush | null = null
  private points: SpatialPoint[] = []

  build(points: SpatialPoint[]): void {
    const index = new KDBush(points.length)
    for (const point of points) index.add(point.x, point.y)
    index.finish()

    this.points = points
    this.index = index
  }

  range(bbox: BBox): QueryResult {
    const index = this.requireIndex()
    const startedAt = performance.now()
    const indices = index.range(bbox.minX, bbox.minY, bbox.maxX, bbox.maxY)
    const elapsedMs = performance.now() - startedAt
    const ids = indices.map((position) => this.points[position].id)
    return { ids, elapsedMs, hitCount: ids.length }
  }

  within(query: RadiusQuery): QueryResult {
    const index = this.requireIndex()
    const startedAt = performance.now()
    const indices = index.within(query.x, query.y, query.radius)
    const elapsedMs = performance.now() - startedAt
    const ids = indices.map((position) => this.points[position].id)
    return { ids, elapsedMs, hitCount: ids.length }
  }

  withinInto(query: RadiusQuery): QueryResult {
    const index = this.requireIndex()
    const output = new Uint32Array(this.points.length)
    const startedAt = performance.now()
    const hitCount = index.withinInto(query.x, query.y, query.radius, output)
    const elapsedMs = performance.now() - startedAt
    const ids = Array.from(output.subarray(0, hitCount), (position) =>
      this.points[position].id,
    )
    return { ids, elapsedMs, hitCount }
  }

  getInfo(): { byteLength: number; numItems: number; nodeSize: number } | null {
    if (!this.index) return null
    return {
      byteLength: this.index.data.byteLength,
      numItems: this.index.numItems,
      nodeSize: this.index.nodeSize,
    }
  }

  getStructureSnapshot(bbox: BBox, maxDepth: number): KDStructureSnapshot {
    const index = this.requireIndex()
    const entries = Array.from(index.ids, (originalIndex, position) => ({
      position,
      originalIndex,
      pointId: this.points[originalIndex].id,
      x: index.coords[position * 2],
      y: index.coords[position * 2 + 1],
    }))
    const splits: KDSplitLine[] = []
    const nodes: KDTreeNode[] = []

    this.collectSplits(
      0,
      index.numItems - 1,
      0,
      0,
      bbox,
      Math.max(0, maxDepth),
      splits,
      nodes,
    )
    return { entries, splits, nodes }
  }

  traceRange(query: BBox, dataBounds: BBox): KDQueryTrace {
    const index = this.requireIndex()
    const trace: KDQueryTrace = {
      visitedNodes: 0,
      prunedNodes: 0,
      checkedPoints: 0,
      totalPoints: index.numItems,
      steps: [],
      matchedIds: [],
    }
    this.traceRangeNode(0, index.numItems - 1, 0, 0, dataBounds, query, trace)
    trace.steps.push({
      kind: 'complete',
      description: `查询完成，累计命中 ${trace.matchedIds.length} 个点。`,
      depth: 0,
      axis: 'x',
      range: [0, Math.max(0, index.numItems - 1)],
      region: { ...dataBounds },
      pointPosition: null,
      pointId: null,
      hit: null,
      newlyMatchedIds: [],
    })
    return trace
  }

  clear(): void {
    this.index = null
    this.points = []
  }

  private requireIndex(): KDBush {
    if (!this.index) throw new Error('请先构建 KDBush 索引。')
    return this.index
  }

  private collectSplits(
    left: number,
    right: number,
    axis: 0 | 1,
    depth: number,
    bounds: BBox,
    maxDepth: number,
    output: KDSplitLine[],
    nodes: KDTreeNode[],
  ): void {
    const index = this.requireIndex()
    if (right < left || right - left <= index.nodeSize || depth >= maxDepth) return

    const middle = (left + right) >> 1
    const x = index.coords[middle * 2]
    const y = index.coords[middle * 2 + 1]
    nodes.push({
      position: middle,
      left,
      right,
      depth,
      axis: axis === 0 ? 'x' : 'y',
    })
    output.push({
      depth,
      axis: axis === 0 ? 'x' : 'y',
      value: axis === 0 ? x : y,
      start: axis === 0 ? [x, bounds.minY] : [bounds.minX, y],
      end: axis === 0 ? [x, bounds.maxY] : [bounds.maxX, y],
    })

    if (axis === 0) {
      this.collectSplits(left, middle - 1, 1, depth + 1, { ...bounds, maxX: x }, maxDepth, output, nodes)
      this.collectSplits(middle + 1, right, 1, depth + 1, { ...bounds, minX: x }, maxDepth, output, nodes)
    } else {
      this.collectSplits(left, middle - 1, 0, depth + 1, { ...bounds, maxY: y }, maxDepth, output, nodes)
      this.collectSplits(middle + 1, right, 0, depth + 1, { ...bounds, minY: y }, maxDepth, output, nodes)
    }
  }

  private traceRangeNode(
    left: number,
    right: number,
    axis: 0 | 1,
    depth: number,
    bounds: BBox,
    query: BBox,
    trace: KDQueryTrace,
  ): void {
    const index = this.requireIndex()
    if (right < left) return
    if (!boxesIntersect(bounds, query)) {
      trace.prunedNodes += 1
      trace.steps.push({
        kind: 'prune',
        description: `区间 [${left}, ${right}] 与 bbox 不相交，跳过整个子树。`,
        depth,
        axis: axis === 0 ? 'x' : 'y',
        range: [left, right],
        region: { ...bounds },
        pointPosition: null,
        pointId: null,
        hit: null,
        newlyMatchedIds: [],
      })
      return
    }

    trace.visitedNodes += 1
    trace.steps.push({
      kind: 'visit',
      description: `访问区间 [${left}, ${right}]，沿 ${axis === 0 ? 'X' : 'Y'} 轴判断。`,
      depth,
      axis: axis === 0 ? 'x' : 'y',
      range: [left, right],
      region: { ...bounds },
      pointPosition: null,
      pointId: null,
      hit: null,
      newlyMatchedIds: [],
    })
    if (right - left <= index.nodeSize) {
      trace.checkedPoints += right - left + 1
      const newlyMatchedIds: string[] = []
      for (let position = left; position <= right; position += 1) {
        const x = index.coords[position * 2]
        const y = index.coords[position * 2 + 1]
        if (x < query.minX || x > query.maxX || y < query.minY || y > query.maxY) continue
        const pointId = this.points[index.ids[position]].id
        newlyMatchedIds.push(pointId)
        trace.matchedIds.push(pointId)
      }
      trace.steps.push({
        kind: 'leaf',
        description: `扫描叶子区间 [${left}, ${right}] 的 ${right - left + 1} 个点，新增命中 ${newlyMatchedIds.length} 个。`,
        depth,
        axis: axis === 0 ? 'x' : 'y',
        range: [left, right],
        region: { ...bounds },
        pointPosition: null,
        pointId: null,
        hit: null,
        newlyMatchedIds,
      })
      return
    }

    const middle = (left + right) >> 1
    const x = index.coords[middle * 2]
    const y = index.coords[middle * 2 + 1]
    trace.checkedPoints += 1
    const hit = x >= query.minX && x <= query.maxX && y >= query.minY && y <= query.maxY
    const originalIndex = index.ids[middle]
    const matchedPointId = this.points[originalIndex].id
    if (hit) trace.matchedIds.push(matchedPointId)
    trace.steps.push({
      kind: 'check',
      description: `检查中位点 ${matchedPointId}：${hit ? '命中 bbox，加入结果' : '未命中'}。`,
      depth,
      axis: axis === 0 ? 'x' : 'y',
      range: [left, right],
      region: { ...bounds },
      pointPosition: middle,
      pointId: matchedPointId,
      hit,
      newlyMatchedIds: hit ? [matchedPointId] : [],
    })
    if (axis === 0) {
      this.traceRangeNode(left, middle - 1, 1, depth + 1, { ...bounds, maxX: x }, query, trace)
      this.traceRangeNode(middle + 1, right, 1, depth + 1, { ...bounds, minX: x }, query, trace)
    } else {
      this.traceRangeNode(left, middle - 1, 0, depth + 1, { ...bounds, maxY: y }, query, trace)
      this.traceRangeNode(middle + 1, right, 0, depth + 1, { ...bounds, minY: y }, query, trace)
    }
  }
}

function boxesIntersect(left: BBox, right: BBox): boolean {
  return !(
    left.maxX < right.minX ||
    left.minX > right.maxX ||
    left.maxY < right.minY ||
    left.minY > right.maxY
  )
}
