# 地理空间数据索引
使用索引可快速访问数据库表中的特定信息。按照传统数据索引的思路，通过建立地理空间对象与计算机存储位置之间的关系，快速提取所选中的地理要素，从而提高地理空间目标操作的速度和效率。与传统数据相比，地理空间数据具有属性、几何形态和时间三个基本特点。对属性数据建立索引可采用传统数据索引的方法见 2.3 节。几何形态数据具有：①几何形状不规则，且实体之间的空间关系复杂（包括相交、相邻、包含等）和存储需求量大；②空间实体的空间操作，如求交、求并等计算的代价比传统的选择或连接操作要复杂得多，并且计算量也大；③空间实体的空间次序难以定义，难以应用传统的排序技术的特点。为适应一维属性数据的主关键字索引而设计的传统数据索引技术，难以直接应用于空间数据索引领域。研究高效的面向空间几何形态数据索引结构与检索算法，也就成为了地理空间数据库的关键技术。



## 开源库对比

| 库 | 索引类型 | 语言 / 生态 | 适合场景 | 优点 | 局限 | 最佳实践 | 仓库地址 |
|---|---|---|---|---|---|---|---|
| KDBush | 扁平 KD-tree 风格静态点索引 | JavaScript | 大量二维点的 bbox 范围查询、半径查询、前端地图点检索 | 代码短，构建和查询快，内存占用低，索引可序列化为紧凑 buffer | 只支持点；静态索引，构建后不能增删；不直接处理球面距离 | 作为第一个源码样本；用随机点、网格点、真实 POI 三类数据做 range/radius benchmark | [mourner/kdbush](https://github.com/mourner/kdbush) |
| geokdbush | KDBush 的地理最近邻扩展 | JavaScript | 经纬度点的最近邻查询，如附近门店、附近车辆、附近 POI | 处理地球曲率和日期变更线，比直接欧氏距离更适合经纬度 | 依赖静态点集；主要解决地理 kNN，不是通用空间谓词库 | 和 KDBush 配套学习；对比平面距离与球面距离在高纬度、跨日期变更线附近的差异 | [mourner/geokdbush](https://github.com/mourner/geokdbush) |
| RBush | 优化 R-tree | JavaScript | 点和矩形对象的动态空间索引、地图可视化、碰撞检测、bbox 相交查询 | 支持动态插入删除；支持 bulk load；生态成熟，适合读 R-tree 工程实现 | 主要基于 bbox 过滤，不等于精确几何谓词；高频更新场景仍需评估重建成本 | 用 `load` 批量构建初始索引；查询后再用精确几何算法二次过滤；单独测试 insert/remove 成本 | [mourner/rbush](https://github.com/mourner/rbush) |
| rbush-knn | RBush 最近邻扩展 | JavaScript | 基于 RBush 的近邻查询 | 能在已有 RBush 索引上补充 kNN 能力 | 不是独立索引；结果依赖 bbox 距离剪枝，复杂地理距离需要额外处理 | 和 RBush 一起 benchmark：bbox 查询、kNN 查询、带过滤条件 kNN 查询分开测 | [mourner/rbush-knn](https://github.com/mourner/rbush-knn) |
| Flatbush | Packed Hilbert R-tree 静态索引 | JavaScript | 海量静态点 / 矩形对象，前端地图、可视化、计算几何预过滤 | 静态索引快且紧凑；Hilbert 排序有利于空间局部性；适合百万级对象 | 构建后不能动态增删；主要做 bbox 查询 | 数据不频繁变化时优先考虑；与 RBush 对比 bulk load、查询耗时和内存占用 | [mourner/flatbush](https://github.com/mourner/flatbush) |
| geoflatbush | Flatbush 的地理最近邻扩展 | JavaScript | 地理 bbox 对象的最近邻查询，如找最近区域、最近道路 bbox 候选 | 兼顾 Flatbush 的静态性能和地理距离处理 | 适合 bbox 级候选召回，精确几何仍需二次判断 | 用于“先召回候选，再精确计算距离”的两阶段查询 | [mourner/geoflatbush](https://github.com/mourner/geoflatbush) |
| kdtree-rs | K 维 KD-tree | Rust | 多维点最近邻、半径搜索、Rust 服务端空间检索 | 类型系统清晰，适合学习泛型、距离度量和最近邻剪枝 | KD-tree 在高维数据上会退化；不适合矩形、线、面对象 | 用二维、三维和更高维数据分别测试；观察维度升高后查询性能变化 | [mrhooray/kdtree-rs](https://github.com/mrhooray/kdtree-rs) |
| davidmoten/rtree | 不可变内存 R-tree | Java | Java 服务端的点 / 矩形索引、响应式查询、线程安全读多写少场景 | 不可变结构便于并发读取；API 清晰；适合理解 R-tree 插入、搜索和结构共享 | 写入会生成新结构；不适合极高频原地更新；主要是内存索引 | 适合读多写少；benchmark 时区分单次插入、批量构建、查询吞吐和 GC 成本 | [davidmoten/rtree](https://github.com/davidmoten/rtree) |
| davidmoten/rtree2 | 不可变二维内存 R-tree | Java | 希望使用更直接 Iterable API 的 Java 二维 R-tree 场景 | 是 `rtree` 的后续版本，API 更轻；已发布到 Maven Central | 聚焦二维；不直接提供数据库级持久化能力 | 新 Java 项目优先看 `rtree2`，旧项目或响应式 API 再看 `rtree` | [davidmoten/rtree2](https://github.com/davidmoten/rtree2) |
| libspatialindex | R*-tree、MVR-tree、TPR-tree | C++ / C API | 工程级 R-tree 实现、Python Rtree 底层、需要动态更新和磁盘序列化的场景 | 功能完整，支持范围查询、点定位、最近邻、kNN、插入删除更新 | C++ 工程复杂度高；直接阅读成本高于前端小库 | 作为 R-tree 工程实现样本；先通过 Python Rtree 使用，再回看 C++ 核心实现 | [libspatialindex/libspatialindex](https://github.com/libspatialindex/libspatialindex) |
| Rtree | libspatialindex 的 Python 封装 | Python GIS | Python 中的空间连接、相交查询、最近邻、磁盘序列化 | 易接入 GeoPandas / Shapely 工作流；功能比轻量 JS 库更完整 | 性能和安装依赖受 libspatialindex 影响；查询通常仍需精确几何二次过滤 | 用作 Python benchmark 基线；记录 build、intersection、nearest、delete、serialization 成本 | [Toblerity/rtree](https://github.com/Toblerity/rtree) |
| Shapely STRtree | Sort-Tile-Recursive R-tree 静态查询索引 | Python / GEOS | Python 几何对象的批量空间谓词预过滤、空间连接候选生成 | 和 Shapely 几何对象天然集成；查询时可结合 predicate；适合静态几何集合 | query-only，构建后不能增删；索引 bbox，复杂几何仍需谓词过滤 | 对面数据、线数据做 `query` + `predicate` benchmark；和 Rtree 对比静态批量查询 | [shapely/shapely](https://github.com/shapely/shapely) |
| H3 | 层级六边形全球离散网格 | C / 多语言绑定 | 全球经纬度聚合、热力分析、邻域搜索、分布式 join、行政区或业务区栅格化 | 编码统一，适合聚合和分桶；多语言生态成熟；六边形邻域关系较自然 | 不是精确几何索引；六边形层级存在少量非完美子划分；边界对象需要覆盖和误差处理 | 用于统计聚合、粗筛和分布式分桶；精确边界判断仍交给几何库或数据库 | [uber/h3](https://github.com/uber/h3) |
| S2 Geometry | 球面几何 + S2 Cell 空间索引 | C++ / Java / 多语言生态 | 全球范围的球面几何、区域覆盖、分布式空间索引、地理围栏 | 原生面向球面；支持将任意区域近似为 S2 Cell 集合；适合大规模地理系统 | 概念和 API 学习成本较高；cell 覆盖是近似，需要控制精度和候选量 | 用来研究“球面空间索引”和“空间填充曲线”；先学 CellId、RegionCoverer、ShapeIndex | [google/s2geometry](https://github.com/google/s2geometry) |
| Supercluster | 基于 KDBush 的层级点聚类索引 | JavaScript | Web 地图大规模点聚类、缩放级别聚合 | 面向地图产品非常成熟；能处理大量点的交互式聚类 | 它是聚类库，不是通用空间索引；适用范围比 KDBush/RBush 窄 | 作为“索引在地图应用中的上层算法”案例；重点看如何用索引支撑不同 zoom 的聚类查询 | [mapbox/supercluster](https://github.com/mapbox/supercluster) |

## Benchmark 设计建议

| 维度 | 建议 |
|---|---|
| 数据类型 | 分别准备随机点、聚簇点、真实 POI、矩形 bbox、线面 bbox。不要只用均匀随机数据。 |
| 数据规模 | 至少测试 `1e4`、`1e5`、`1e6` 三档；如果浏览器内测试，额外记录页面卡顿和内存峰值。 |
| 查询类型 | bbox 范围查询、半径查询、kNN 查询、批量查询、插入、删除、重建索引。 |
| 指标 | 构建耗时、单次查询耗时、批量查询吞吐、内存占用、索引大小、候选集数量、精确过滤后命中数量。 |
| 对照组 | 必须保留 brute force 全量扫描作为基线，否则很难解释索引收益。 |
| 结论方式 | 不只写“谁最快”，而是总结“在哪类数据、哪类查询、哪类更新频率下更合适”。 |

## 阶段性结论

- 如果目标是理解空间索引本体，优先读 `kdbush`、`rbush`、`flatbush`，它们足够小，能快速建立源码级理解。
- 如果目标是 Python GIS 实验，优先用 Shapely `STRtree` 和 `Rtree`，它们更接近 GeoPandas 工作流。
- 如果目标是服务端工程和并发读，Java `rtree2`、Rust `kdtree-rs`、C++ `libspatialindex` 更值得深入。
- 如果目标是全球经纬度数据聚合和分布式检索，重点研究 `H3` 和 `S2`，它们和传统平面 R-tree/KD-tree 是不同路线。
- PostGIS 建议放到第二阶段，用同样的数据和查询类型验证 GiST、SP-GiST、BRIN、查询计划和 SQL 写法对性能的影响。

## 参考文献

> 部分内容整理自 《地理空间数据库原理（第二版）》崔铁军。谨向作者致敬并致谢；原书著作权及相关权利归作者及权利人所有。本目录仅用于学习与索引，不替代原书。
