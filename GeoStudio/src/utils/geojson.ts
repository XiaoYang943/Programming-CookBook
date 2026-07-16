import type { SpatialPoint } from '../indexers/types'

export interface PointGeometry {
  type: 'Point'
  coordinates: [number, number]
}

export interface PointFeature {
  type: 'Feature'
  id?: string | number
  properties: Record<string, unknown> | null
  geometry: PointGeometry
}

export interface PointFeatureCollection {
  type: 'FeatureCollection'
  features: PointFeature[]
}

export interface ParsedPointDataset {
  featureCollection: PointFeatureCollection
  points: SpatialPoint[]
  totalFeatureCount: number
  invalidFeatureCount: number
  bounds: [number, number, number, number] | null
}

interface UnknownFeatureCollection {
  type?: unknown
  features?: unknown
}

export function parsePointGeoJSON(text: string): ParsedPointDataset {
  let parsed: unknown
  try {
    parsed = JSON.parse(text)
  } catch {
    throw new Error('文件不是有效的 JSON。')
  }

  if (!isRecord(parsed)) throw new Error('GeoJSON 根节点必须是对象。')
  const collection = parsed as UnknownFeatureCollection
  if (collection.type !== 'FeatureCollection' || !Array.isArray(collection.features)) {
    throw new Error('首版只支持 GeoJSON FeatureCollection。')
  }

  const features: PointFeature[] = []
  const points: SpatialPoint[] = []
  const usedIds = new Set<string>()
  let invalidFeatureCount = 0
  let minX = Infinity
  let minY = Infinity
  let maxX = -Infinity
  let maxY = -Infinity

  collection.features.forEach((candidate, sourceIndex) => {
    if (!isRecord(candidate) || candidate.type !== 'Feature') {
      invalidFeatureCount += 1
      return
    }

    const geometry = candidate.geometry
    if (!isRecord(geometry) || geometry.type !== 'Point') {
      invalidFeatureCount += 1
      return
    }

    const coordinates = geometry.coordinates
    if (
      !Array.isArray(coordinates) ||
      coordinates.length < 2 ||
      !isFiniteNumber(coordinates[0]) ||
      !isFiniteNumber(coordinates[1]) ||
      coordinates[0] < -180 ||
      coordinates[0] > 180 ||
      coordinates[1] < -90 ||
      coordinates[1] > 90
    ) {
      invalidFeatureCount += 1
      return
    }

    const id = createUniqueFeatureId(candidate.id, sourceIndex, usedIds)
    const properties = isRecord(candidate.properties) ? candidate.properties : {}
    const feature: PointFeature = {
      type: 'Feature',
      id,
      properties: { ...properties, __geostudio_id: id },
      geometry: {
        type: 'Point',
        coordinates: [coordinates[0], coordinates[1]],
      },
    }

    features.push(feature)
    points.push({ id, x: coordinates[0], y: coordinates[1], properties })
    minX = Math.min(minX, coordinates[0])
    minY = Math.min(minY, coordinates[1])
    maxX = Math.max(maxX, coordinates[0])
    maxY = Math.max(maxY, coordinates[1])
  })

  if (features.length === 0) {
    throw new Error('文件中没有有效的 WGS84 Point Feature。')
  }

  return {
    featureCollection: { type: 'FeatureCollection', features },
    points,
    totalFeatureCount: collection.features.length,
    invalidFeatureCount,
    bounds: [minX, minY, maxX, maxY],
  }
}

export function selectFeaturesByIds(
  collection: PointFeatureCollection,
  ids: string[],
): PointFeatureCollection {
  const selectedIds = new Set(ids)
  return {
    type: 'FeatureCollection',
    features: collection.features.filter((feature) =>
      selectedIds.has(String(feature.properties?.__geostudio_id)),
    ),
  }
}

function createUniqueFeatureId(
  id: unknown,
  sourceIndex: number,
  usedIds: Set<string>,
): string {
  const baseId =
    typeof id === 'string' || typeof id === 'number'
      ? String(id)
      : `feature-${sourceIndex}`
  let uniqueId = baseId
  let suffix = 1
  while (usedIds.has(uniqueId)) uniqueId = `${baseId}-${suffix++}`
  usedIds.add(uniqueId)
  return uniqueId
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value)
}

function isFiniteNumber(value: unknown): value is number {
  return typeof value === 'number' && Number.isFinite(value)
}
