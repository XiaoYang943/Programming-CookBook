fn apply_to_all(numbers: &[i32], operation: impl Fn(i32) -> i32) -> Vec<i32> {
    let mut results = Vec::new();

    for number in numbers {
        results.push(operation(*number));
    }

    results
}

fn main() {
    let numbers = vec![1, 2, 3, 4];
    let factor = 10;

    // `multiply` 保存的是“把数字乘以 factor”这个行为，
    // 不是某一次计算出来的结果。
    let multiply = |number| number * factor;

    let results = apply_to_all(&numbers, multiply);

    println!("{:?}", results);
    assert_eq!(results, vec![10, 20, 30, 40]);
}