use std::fmt::{Debug, Display};

trait Printable: Debug + Display {}
impl<T: Debug + Display> Printable for T {}

fn print_value(value: &impl Printable) -> String {
    format!("{value} / {value:?}")
}

fn main() {
    assert_eq!(print_value(&42), "42 / 42");
}
