fn double(values: &mut [i32]) {
    for value in values {
        *value *= 2;
    }
}

fn main() {
    let mut values = [1, 2, 3];
    double(&mut values);

    assert_eq!(values, [2, 4, 6]);
}
