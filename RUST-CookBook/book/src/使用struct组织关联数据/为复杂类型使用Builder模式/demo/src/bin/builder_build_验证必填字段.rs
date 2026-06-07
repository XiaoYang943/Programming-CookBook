struct User {
    name: String,
}

#[derive(Default)]
struct UserBuilder {
    name: Option<String>,
}

impl UserBuilder {
    fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    fn build(self) -> Result<User, &'static str> {
        self.name.map(|name| User { name }).ok_or("缺少 name")
    }
}

fn main() {
    assert_eq!(UserBuilder::default().name("Alice").build().unwrap().name, "Alice");
    assert!(UserBuilder::default().build().is_err());
}
