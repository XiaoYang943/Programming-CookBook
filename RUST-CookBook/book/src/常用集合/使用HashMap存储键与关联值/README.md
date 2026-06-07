# 使用 HashMap 存储键与关联值

> 《Rust 程序设计语言》原文：[使用 HashMap 存储键与关联值](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)。

## 使用 `HashMap<K, V>` 存储关联值

{{#playground demo/src/bin/hashmap_插入与读取关联值.rs editable}}

## 使用 `entry` 按需插入或更新

{{#playground demo/src/bin/hashmap_entry_按需插入与计数.rs editable}}

## 理解键值所有权

实现 `Copy` 的值会复制进入 HashMap；拥有类型会移动。传入引用时，引用必须保持有效。

{{#playground demo/src/bin/hashmap_移动拥有类型.rs editable}}

