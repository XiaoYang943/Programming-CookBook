trait Value { fn value(&self) -> i32; }
impl Value for i32 { fn value(&self) -> i32 { *self } }
impl Value for bool { fn value(&self) -> i32 { i32::from(*self) } }

fn read(value: &dyn Value) -> i32 { value.value() }

fn main() {
    let number = 42;
    let flag = true;
    assert_eq!(read(&number) + read(&flag), 43);
}
