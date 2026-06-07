trait Value { fn value(&self) -> i32; }
struct Number(i32);
impl Value for Number { fn value(&self) -> i32 { self.0 } }

fn create() -> impl Value { Number(42) }
fn read(value: &impl Value) -> i32 { value.value() }

fn main() {
    assert_eq!(read(&create()), 42);
}
