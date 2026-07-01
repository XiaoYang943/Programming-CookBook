trait Named {
    fn name(&self) -> &str;
    fn greeting(&self) -> String { format!("Hello, {}", self.name()) }
}

struct User(&'static str);
impl Named for User { fn name(&self) -> &str { self.0 } }

fn main() {
    assert_eq!(User("Alice").greeting(), "Hello, Alice");
}
