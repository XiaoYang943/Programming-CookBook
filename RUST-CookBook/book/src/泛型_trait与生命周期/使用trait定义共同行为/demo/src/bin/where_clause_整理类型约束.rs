use std::fmt::{Debug, Display};

fn describe<T>(value: &T) -> String
where
    T: Debug + Display,
{
    format!("{value} / {value:?}")
}

fn main() {
    assert_eq!(describe(&42), "42 / 42");
}
