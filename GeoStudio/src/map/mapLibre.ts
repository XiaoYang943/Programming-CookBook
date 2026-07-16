import maplibregl, { type Map } from 'maplibre-gl'

export type GeoStudioMap = Map

const DEFAULT_STYLE_URL = 'https://demotiles.maplibre.org/style.json'

const DEFAULT_VIEW = {
  center: [104.1954, 35.8617] as [number, number],
  zoom: 3.2,
}

export function createMap(container: HTMLElement): GeoStudioMap {
  const map = new maplibregl.Map({
    container,
    style: DEFAULT_STYLE_URL,
    center: DEFAULT_VIEW.center,
    zoom: DEFAULT_VIEW.zoom,
    attributionControl: false,
  })

  map.addControl(new maplibregl.NavigationControl(), 'top-right')
  map.addControl(
    new maplibregl.AttributionControl({ compact: true }),
    'bottom-right',
  )

  return map
}

