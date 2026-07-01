# 泛型 enum 实践

泛型 enum 适合表达“状态集合固定，但状态携带的数据类型可以变化”的业务模型。

## 使用 `Result<T, E>` 表达接口调用结果

`Result<T, E>` 强制调用方同时处理成功数据和失败原因，适合数据库访问、HTTP
请求、文件操作等可能失败的边界。

{{#playground demo/src/bin/api_result.rs editable}}

## 使用 `Option<T>` 表达可选配置

`Option<T>` 明确区分“有配置”和“没有配置”，避免使用空字符串、`0` 等特殊值
表示缺失。默认值通常应在配置读取边界统一补全。

{{#playground demo/src/bin/optional_config.rs editable}}

## 使用泛型 enum 表达异步任务状态

任务状态通常不只有成功和失败，还包括等待、运行中等中间状态。将状态建模为
enum 可以避免无效组合，例如“任务既完成又失败”。

{{#playground demo/src/bin/async_task_state.rs editable}}

## 使用泛型 enum 表达分页查询结果

分页成功时需要携带业务数据和下一页游标，失败时需要携带错误。泛型参数让用户、
订单等不同业务对象复用同一个分页协议和错误处理流程。

{{#playground demo/src/bin/paginated_result.rs editable}}

## 使用泛型 enum 表达缓存查询结果

缓存查询通常需要区分命中、未命中和数据过期。相比 `Option<T>`，独立状态可以让
调用方针对过期数据采用“先返回旧值，再后台刷新”等策略。

{{#playground demo/src/bin/cache_result.rs editable}}

## 选择原则

- 只有“存在”和“不存在”两种状态时，使用 `Option<T>`。
- 操作只有“成功”和“失败”两种结果时，使用 `Result<T, E>`。
- 业务存在三个或更多有明确含义的状态时，定义领域 enum。
- 为每个变体携带处理该状态真正需要的数据，避免额外的布尔字段。
