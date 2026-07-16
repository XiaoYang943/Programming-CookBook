import type {
  BBox,
  PointSpatialIndexer,
  QueryResult,
  RadiusQuery,
  SpatialPoint,
} from './types'

export class BruteForceIndexer implements PointSpatialIndexer {
  readonly name = 'Brute force'

  private points: SpatialPoint[] = []

  build(points: SpatialPoint[]): void {
    this.points = points
  }

  range(bbox: BBox): QueryResult {
    const startedAt = performance.now()
    const ids = this.points
      .filter(
        (point) =>
          point.x >= bbox.minX &&
          point.x <= bbox.maxX &&
          point.y >= bbox.minY &&
          point.y <= bbox.maxY,
      )
      .map((point) => point.id)
    return {
      ids,
      elapsedMs: performance.now() - startedAt,
      hitCount: ids.length,
    }
  }

  within(query: RadiusQuery): QueryResult {
    const radiusSquared = query.radius * query.radius
    const startedAt = performance.now()
    const ids = this.points
      .filter((point) => {
        const dx = point.x - query.x
        const dy = point.y - query.y
        return dx * dx + dy * dy <= radiusSquared
      })
      .map((point) => point.id)
    return {
      ids,
      elapsedMs: performance.now() - startedAt,
      hitCount: ids.length,
    }
  }
}
