trait Describe { fn describe(&self) -> &'static str; }
struct User;
impl Describe for User { fn describe(&self) -> &'static str { "user" } }

fn main() {
    assert_eq!(User.describe(), "user");
}
