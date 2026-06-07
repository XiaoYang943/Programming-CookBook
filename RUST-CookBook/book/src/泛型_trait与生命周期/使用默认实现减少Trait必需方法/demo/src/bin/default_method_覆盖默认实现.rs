trait Greeting {
    fn greeting(&self) -> &'static str { "Hello" }
}

struct Formal;
impl Greeting for Formal {
    fn greeting(&self) -> &'static str { "Good morning" }
}

fn main() {
    assert_eq!(Formal.greeting(), "Good morning");
}
