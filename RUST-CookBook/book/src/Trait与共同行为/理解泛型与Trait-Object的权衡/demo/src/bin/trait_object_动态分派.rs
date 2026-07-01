trait Value { fn value(&self) -> i32; }
impl Value for i32 { fn value(&self) -> i32 { *self } }
impl Value for bool { fn value(&self) -> i32 { i32::from(*self) } }

fn main() {
    let values: Vec<Box<dyn Value>> = vec![Box::new(42), Box::new(true)];
    assert_eq!(values.iter().map(|value| value.value()).sum::<i32>(), 43);
}
