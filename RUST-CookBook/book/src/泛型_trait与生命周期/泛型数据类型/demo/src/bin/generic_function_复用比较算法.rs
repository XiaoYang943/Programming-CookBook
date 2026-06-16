fn bubble_sort<T>(values: &mut [T])
where
    T: PartialOrd,
{
    for unsorted_end in (1..values.len()).rev() {
        for index in 0..unsorted_end {
            if values[index] > values[index + 1] {
                values.swap(index, index + 1);
            }
        }
    }
}

fn main() {
    let mut scores = [85, 60, 100, 90];
    bubble_sort(&mut scores);
    assert_eq!(scores, [60, 85, 90, 100]);

    let mut letters = ['z', 'a', 'c', 'b'];
    bubble_sort(&mut letters);
    assert_eq!(letters, ['a', 'b', 'c', 'z']);

    let mut fruits = ["pear", "apple", "orange"];
    bubble_sort(&mut fruits);
    assert_eq!(fruits, ["apple", "orange", "pear"]);

    assert_eq!(f64::NAN.partial_cmp(&1.0), None);
}
