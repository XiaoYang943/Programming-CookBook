use std::fmt;

trait Outline: fmt::Display {
    fn outline(&self) -> String { format!("*** {} ***", self) }
}

struct Name(&'static str);
impl fmt::Display for Name { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) } }
impl Outline for Name {}

fn main() {
    assert_eq!(Name("Rust").outline(), "*** Rust ***");
}
