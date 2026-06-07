trait Named { fn name(&self) -> &str; }
struct User(&'static str);
impl Named for User { fn name(&self) -> &str { self.0 } }

fn greeting<T: Named>(value: &T) -> String {
    format!("Hello, {}", value.name())
}

fn main() {
    assert_eq!(greeting(&User("Alice")), "Hello, Alice");
}
