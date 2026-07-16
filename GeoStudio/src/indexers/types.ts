export interface SpatialPoint {
  id: string
  x: number
  y: number
  properties?: Record<string, unknown>
}

export interface BBox {
  minX: number
  minY: number
  maxX: number
  maxY: number
}

export interface RadiusQuery {
  x: number
  y: number
  radius: number
}

export interface QueryResult {
  ids: string[]
  elapsedMs: number
  hitCount: number
}

export interface PointSpatialIndexer {
  readonly name: string
  build(points: SpatialPoint[]): void
  range(bbox: BBox): QueryResult
  within(query: RadiusQuery): QueryResult
}

export type QueryType = 'bbox' | 'radius' | 'radius-into'

export interface BenchmarkRecord {
  id: number
  datasetName: string
  pointCount: number
  queryType: QueryType
  parameters: string
  kdbushMs: number
  bruteForceMs: number
  kdbushHits: number
  bruteForceHits: number
  consistent: boolean
  createdAt: string
}

export interface KDIndexEntry {
  position: number
  originalIndex: number
  pointId: string
  x: number
  y: number
}

export interface KDSplitLine {
  depth: number
  axis: 'x' | 'y'
  value: number
  start: [number, number]
  end: [number, number]
}

export interface KDStructureSnapshot {
  entries: KDIndexEntry[]
  splits: KDSplitLine[]
  nodes: KDTreeNode[]
}

export interface KDQueryTrace {
  visitedNodes: number
  prunedNodes: number
  checkedPoints: number
  totalPoints: number
  steps: KDTraceStep[]
  matchedIds: string[]
}

export interface KDTreeNode {
  position: number
  left: number
  right: number
  depth: number
  axis: 'x' | 'y'
}

export interface KDTraceStep {
  kind: 'visit' | 'check' | 'leaf' | 'prune' | 'complete'
  description: string
  depth: number
  axis: 'x' | 'y'
  range: [number, number]
  region: BBox
  pointPosition: number | null
  pointId: string | null
  hit: boolean | null
  newlyMatchedIds: string[]
}
