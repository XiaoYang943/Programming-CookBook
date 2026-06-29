fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let min = 3;

    let even_nums: Vec<i32> = nums
        .into_iter()
        .filter(|n| *n > min && n % 2 == 0)
        .collect();

    println!("{:?}", even_nums);
    assert_eq!(even_nums, vec![4, 6]);
}