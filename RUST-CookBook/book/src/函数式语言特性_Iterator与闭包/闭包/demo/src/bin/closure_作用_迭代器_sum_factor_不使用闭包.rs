fn calculate(n: i32, sum: &mut i32, factor: i32) -> i32 {
    *sum += n;
    *sum * factor
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let factor = 10;
    let mut sum = 0;
    let mut results = Vec::new();

    // 只有 `factor` 时，可以用 `zip(std::iter::repeat(factor))`
    // 把固定参数塞进每个迭代项里。
    //
    // 但 `sum` 不是固定参数，它是迭代过程中的状态：
    // 0 -> 1 -> 3 -> 6 -> 10。
    // 每一步的 `sum` 都依赖上一步的结果，不能提前用 `repeat` 或 `zip` 准备好。
    //
    // 如果不用闭包，又要让每一步都能修改同一个 `sum`，这里就只能退回显式循环。
    for n in nums.into_iter() {
        results.push(calculate(n, &mut sum, factor));
    }

    println!("{:?}", results);
    assert_eq!(results, vec![10, 30, 60, 100]);
}