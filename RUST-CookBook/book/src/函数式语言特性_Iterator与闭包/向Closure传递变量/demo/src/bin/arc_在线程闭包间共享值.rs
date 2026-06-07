use std::sync::Arc;
use std::thread;

fn main() {
    let values = Arc::new(vec![1, 2, 3, 4]);

    let first_values = Arc::clone(&values);
    let first = thread::spawn(move || first_values.iter().sum::<i32>());

    let second_values = Arc::clone(&values);
    let second = thread::spawn(move || second_values.len());

    assert_eq!(first.join().unwrap(), 10);
    assert_eq!(second.join().unwrap(), 4);
    assert_eq!(Arc::strong_count(&values), 1);
}
