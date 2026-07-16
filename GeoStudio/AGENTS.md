# GeoStudio Desktop GIS 实验平台约定

## 项目定位

GeoStudio 是一个桌面端 GIS 实验平台，采用分阶段方式建设。

当前项目不以替代 QGIS、GeoLibre 或 PostGIS 等通用 GIS 平台为目标。第一阶段先建立稳定、可运行的桌面地图应用骨架；第二阶段再围绕 `kdbush` 建立“本地数据加载 -> 地图显示 -> 构建索引 -> 空间查询 -> 性能对比 -> 结果可视化”的完整验证闭环。

后续可以继续扩展 `geokdbush`、`rbush`、`flatbush`、DuckDB Spatial、PostGIS、空间分析、数据格式转换、图层样式编辑和 benchmark 报告等能力。

## 技术栈

- 桌面框架：Tauri v2
- 前端框架：Vue 3
- 语言：TypeScript
- 构建工具：Vite
- 地图渲染：MapLibre GL JS
- 状态管理：Pinia（第二阶段按实际状态复杂度引入）
- 空间索引库：KDBush（第二阶段引入）

## 阶段划分原则

- 必须先完成第一阶段并通过验收，再开始第二阶段。
- 第一阶段只搭建桌面 GIS 应用骨架，不提前实现 KDBush、GeoJSON 数据管理或 benchmark 功能。
- 第二阶段建立 KDBush 功能验证闭环，不扩展为完整 GIS 平台。
- 每个阶段都应保持应用可以启动、可以构建，并避免为了后续功能过度设计。

## 第一阶段：Tauri + Vue + MapLibre 桌面地图骨架

### 阶段目标

搭建基于 Tauri v2、Vue 3、TypeScript 和 Vite 的 Windows 桌面应用，并集成 MapLibre GL JS。

第一阶段的最终目标是：能够构建出 Windows 可执行程序，启动后直接进入地图界面，并正常显示、缩放和拖动 MapLibre 地图。

### 必须支持

- 初始化 Tauri v2 + Vue 3 + TypeScript + Vite 项目。
- 集成 MapLibre GL JS。
- 应用启动后直接进入地图界面，不显示营销式首页或无关欢迎页。
- 地图铺满主要工作区域。
- 地图可以正常加载底图。
- 支持地图缩放和拖动等基础交互。
- 地图容器能够随桌面窗口尺寸变化正确调整。
- 开发模式下能够正常启动应用。
- 能够完成 Windows 桌面程序构建，并生成可启动的 `.exe` 文件。

### 暂不实现

- 本地 GeoJSON 文件加载。
- 点图层管理。
- KDBush 索引构建。
- bbox 查询和 radius 查询。
- brute force 对照查询。
- 查询结果高亮。
- benchmark 记录和报告。
- 完整的左右侧栏及底部面板。
- 空间投影转换。
- 图层样式编辑。
- 项目保存和恢复。

### 界面要求

- 应用启动后直接显示地图。
- 第一阶段可以只包含一个地图主视图和必要的地图控件。
- 不要求提前搭建第二阶段的空面板。
- 默认地图中心点、缩放级别和底图样式应集中配置，避免散落在组件中。
- 如果使用在线底图，必须明确其网络依赖；底图加载失败时应用不应崩溃。

### 建议目录结构

```text
src/
  components/
    MapView.vue
  map/
    mapLibre.ts
  App.vue
  main.ts
src-tauri/
```

### 工程约定

- 使用 Vue 3 `<script setup lang="ts">`。
- MapLibre 地图实例集中封装，不要在多个组件中重复创建地图。
- 组件卸载时必须释放 MapLibre 地图实例。
- MapLibre 样式文件应在统一入口引入。
- 不把令牌、账号、敏感路径或本地隐私数据写入仓库。
- 如果底图服务需要访问令牌，必须通过环境变量或本地配置提供，不得硬编码。

### 第一阶段验收标准

第一阶段完成时必须满足：

- `npm` 开发命令能够正常启动 Vue 前端。
- Tauri 开发模式能够正常打开桌面窗口。
- 桌面窗口启动后直接显示 MapLibre 地图。
- 地图可以正常缩放、拖动和调整窗口尺寸。
- 地图实例不会因 Vue 组件重复渲染而重复创建。
- 能够成功执行 Tauri Windows 构建。
- 构建产物中存在可以独立启动的 `.exe` 文件。
- 启动 `.exe` 后能够进入地图界面，且没有阻断使用的运行时错误。

## 第二阶段：KDBush 空间索引功能验证

### 阶段目标

在第一阶段桌面地图骨架上实现 KDBush 静态二维点索引验证，形成从本地数据加载到查询性能对比和结果可视化的完整闭环。

### 必须支持

- 通过 Tauri 原生文件选择器打开本地 GeoJSON 点数据。
- 在 MapLibre 地图上显示点图层。
- 从点数据中提取 `[x, y]` 坐标并构建 KDBush 索引。
- 显示索引构建耗时、点数量和索引状态。
- 集成 KDBush 的 `add()` 和 `finish()` 索引构建流程。
- 支持 `range()` bbox 范围查询。
- 支持 `within()` radius 半径查询。
- 支持 `withinInto()` 低分配半径查询实验。
- 显示 `data.byteLength`、`numItems` 和 `nodeSize` 等索引信息。
- 支持 brute force 全量扫描作为对照组。
- 对比 KDBush 查询和 brute force 查询的耗时及命中数量。
- 验证 KDBush 与 brute force 的命中 ID 集合是否一致，不要求结果顺序一致。
- 在地图上高亮查询结果。
- 在面板中记录每次查询的 benchmark 结果。
- 首版先完成单次查询 benchmark；批量随机测试、平均值、最小值、最大值和 P95 作为后续优化项。

### 本地文件选择与读取

- 使用 Tauri Dialog 插件打开操作系统原生文件选择窗口。
- 文件过滤器只显示 `.geojson` 和 `.json` 文件。
- 用户取消选择时不报错，也不清空或改变当前数据集。
- 读取失败、JSON 解析失败或格式不符合要求时，必须显示明确提示。
- 文件只在本地读取和处理，不上传到网络服务。
- 首版采用最直接的文本读取和同步 JSON 解析方式，优先跑通完整流程。

### GeoJSON 输入边界

- 第二阶段首版只接受 WGS84（EPSG:4326）GeoJSON 点数据。
- 主要输入格式为 `FeatureCollection<Point>`。
- 非点几何不能进入 KDBush 索引，并应向用户显示明确提示。
- 无效坐标、空 geometry 或无法解析的 Feature 不得导致应用崩溃。
- Feature 没有可用 `id` 时，应生成在当前数据集内稳定且唯一的 ID。
- 暂不支持坐标投影转换和复杂几何对象索引。

### KDBush 使用边界

KDBush 只适合静态二维点索引。

- 索引构建后不支持动态插入、删除或更新；数据变化后必须重新构建索引。
- bbox 查询使用闭区间边界，即边界上的点计入结果。
- 第二阶段的基础 radius 查询使用二维欧氏距离，并以 `distance <= radius` 判断命中。
- 对 WGS84 经纬度数据执行基础 radius 查询时，半径单位为“度”，不代表米或千米。
- 界面和文档必须明确说明上述距离限制，避免将经纬度差值误认为真实地理距离。
- 真实地理距离和球面最近邻查询留到后续通过 `geokdbush` 或 Turf.js 实现。
- 首版不实现 `SharedArrayBuffer`、Web Worker、索引文件保存、`KDBush.from()` 恢复、多种 TypedArray 对比和 `nodeSize` 调优实验。

### 建议目录结构

```text
src/
  components/
    MapView.vue
    LayerPanel.vue
    IndexPanel.vue
    QueryPanel.vue
    BenchmarkPanel.vue
  stores/
    layerStore.ts
    indexStore.ts
    queryStore.ts
    benchmarkStore.ts
  indexers/
    types.ts
    kdbushIndexer.ts
    bruteForceIndexer.ts
  map/
    mapLibre.ts
    sources.ts
    layers.ts
  utils/
    geojson.ts
    measure.ts
    coordinates.ts
src-tauri/
```

### 核心数据模型

应定义统一的点索引接口，为后续接入 `rbush`、`flatbush` 等索引保留清晰边界。

```ts
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
  name: string
  build(points: SpatialPoint[]): Promise<void> | void
  range(bbox: BBox): QueryResult
  within(query: RadiusQuery): QueryResult
}
```

### UI 布局要求

第二阶段采用桌面 GIS 常见布局：

- 左侧：数据与图层面板。
- 中间：MapLibre 地图。
- 右侧：索引构建与查询参数面板。
- 底部：benchmark 结果表格。

应用启动后仍应直接进入 GIS 工作界面，不增加营销式首页。

当前布局用于跑通第二阶段功能闭环，不视为长期稳定架构，后续允许根据实际使用体验整体重构。

### 地图交互要求

- bbox 查询应支持地图框选，并提供手动输入 bbox 的备用方式。
- radius 查询应支持点击地图选择圆心，并提供输入半径的方式。
- 查询结果必须与原始点图层区分显示，例如使用不同颜色、大小或独立高亮图层。
- 每次查询后保留地图视图状态，不应无故重置中心点和缩放级别。

### Benchmark 要求

每次查询至少记录：

- 数据集名称。
- 点数量。
- 查询类型。
- 查询参数。
- KDBush 查询耗时。
- brute force 查询耗时。
- KDBush 命中数量。
- brute force 命中数量。
- 两者结果是否一致。
- 查询时间。

计时必须遵守：

- benchmark 计时使用 `performance.now()`。
- 查询耗时只统计查询计算，不包含文件解析、地图渲染和结果高亮时间。
- 首版记录单次查询结果，用于验证功能和初步观察性能，不以此形成最终性能结论。
- 批量随机查询、固定随机种子、P95 计算和多数据规模实验在流程跑通后统一设计。

### 第二阶段工程约定

- 所有核心数据结构必须显式声明 TypeScript 类型。
- KDBush 相关逻辑放在 `src/indexers/kdbushIndexer.ts`，不要散落在 Vue 组件中。
- brute force 对照逻辑放在 `src/indexers/bruteForceIndexer.ts`。
- 点数据、原始 Feature 和索引输入应区分管理，避免在前端状态中保存过大的冗余 GeoJSON 副本。
- 首版允许在主线程同步读取、解析、构建索引和查询，优先完成端到端功能闭环。
- 界面应显示文件加载、索引构建或查询状态，避免用户误认为操作没有响应。
- Web Worker、流式解析、分块渲染、Rust 解析、内存优化及大数据专用模式均作为后续优化项。
- 大数据优化需要综合考虑文件大小、点数量、JSON 解析、内存副本、地图渲染、索引构建以及线程通信成本，不在首版中提前限定单一技术方案。

### 第二阶段验收标准

第二阶段完成时必须满足：

- 能加载有效的 WGS84 GeoJSON 点数据并显示在地图上。
- 能成功构建 KDBush 索引并显示构建耗时及点数量。
- bbox 和 radius 查询均能通过 KDBush 与 brute force 执行。
- 相同查询参数下，两种实现的命中 ID 集合一致。
- 查询结果可以在地图上高亮。
- 单次 bbox、`within()` 和 `withinInto()` 查询能够生成 benchmark 记录。
- 界面和文档明确说明 KDBush 只支持静态二维点索引。
- 界面和文档明确说明基础 radius 查询使用坐标单位，而非真实地理距离。
- 用户取消文件选择、选择非法文件或遇到无效 Feature 时，应用不会崩溃并能提供明确反馈。

## 第二阶段暂不要求

- 多格式空间数据完整支持。
- 空间投影转换。
- 复杂几何对象索引。
- 图层样式编辑器。
- PostGIS 或 DuckDB Spatial 集成。
- 插件系统。
- 完整项目保存和恢复。
- Web Worker 和 SharedArrayBuffer。
- 索引文件持久化与 `KDBush.from()` 恢复。
- 多种 TypedArray 和 `nodeSize` 参数调优。
- `1e4`、`1e5`、`1e6` 的系统化性能报告。

## 后续扩展方向

- 接入 `geokdbush`，处理真实经纬度最近邻和球面距离。
- 接入 `rbush`，验证动态 R-tree、矩形对象索引和插入删除成本。
- 接入 `flatbush`，验证静态 Packed Hilbert R-tree 的构建速度、查询速度和内存优势。
- 完善 Web Worker，避免大数据构建索引和查询时阻塞 UI。
- 增加 DuckDB Spatial，用 SQL 做空间查询对照。
- 增加 PostGIS 实验模块，用真实数据库验证 GiST、SP-GiST、BRIN 和查询计划。
- 增加 benchmark 报告导出。
