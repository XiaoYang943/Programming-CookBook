fn is_even_and_greater_than_min(n: i32, min: i32) -> bool {
    n > min && n % 2 == 0
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    let min = 3;

    let mut even_nums = Vec::new();
    for n in nums {
        if is_even_and_greater_than_min(n, min) {
            even_nums.push(n);
        }
    }

    println!("{:?}", even_nums);
    assert_eq!(even_nums, vec![4, 6]);
}