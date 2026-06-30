fn main() {
    let nums = vec![1, 2, 3, 4];
    let factor = 10;
    let mut sum = 0;

    let results: Vec<i32> = nums
        .into_iter()
        .map(|n| {
            // 闭包接收迭代器传入的 `n`。
            // 同时，它还能捕获外部的 `factor`，并修改外部的 `sum`。
            sum += n;
            sum * factor
        })
        .collect();

    println!("{:?}", results);
    assert_eq!(results, vec![10, 30, 60, 100]);
}