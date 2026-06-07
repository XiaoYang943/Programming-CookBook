fn main() {
    let values = vec![1, 2, 3];
    let view: &[i32] = &values;
    assert_eq!(view, [1, 2, 3]);
}
