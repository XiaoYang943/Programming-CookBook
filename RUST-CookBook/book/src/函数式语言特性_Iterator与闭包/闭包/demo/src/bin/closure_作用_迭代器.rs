fn main() {
    let nums = vec![1, 2, 3, 4];
    let factor = 10;

    let results: Vec<i32> = nums
        .into_iter()
        .map(|n| n * factor)    // 闭包可以直接在 `map` 中捕获 `factor`
        .collect();

    println!("{:?}", results);
    assert_eq!(results, vec![10, 20, 30, 40]);
}