trait Value { fn value(&self) -> i32; }
impl Value for i32 { fn value(&self) -> i32 { *self } }

fn read<T: Value>(value: &T) -> i32 { value.value() }

fn main() {
    assert_eq!(read(&42), 42);
}
