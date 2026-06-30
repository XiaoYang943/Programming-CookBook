fn multiply_by_factor((n, factor): (i32, i32)) -> i32 {
    n * factor
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let factor = 10;

    let results: Vec<i32> = nums
        .into_iter()
        .zip(std::iter::repeat(factor)) // 先用 zip 把 factor 塞进每个迭代项里
        .map(multiply_by_factor)    // map 每次只能传一个迭代项，所以需要提前处理 factor
        .collect();

    println!("{:?}", results);
    assert_eq!(results, vec![10, 20, 30, 40]);
}