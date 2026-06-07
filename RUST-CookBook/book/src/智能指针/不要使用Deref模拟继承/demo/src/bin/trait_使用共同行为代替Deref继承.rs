trait Named {
    fn name(&self) -> &str;
}

struct User { name: String }

impl Named for User {
    fn name(&self) -> &str { &self.name }
}

fn greeting(value: &impl Named) -> String {
    format!("Hello, {}", value.name())
}

fn main() {
    assert_eq!(greeting(&User { name: String::from("Alice") }), "Hello, Alice");
}
