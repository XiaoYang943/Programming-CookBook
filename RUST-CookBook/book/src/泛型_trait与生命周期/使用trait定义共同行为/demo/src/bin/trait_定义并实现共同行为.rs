trait Summary {
    fn summarize(&self) -> String;
}

struct Article { title: String }

impl Summary for Article {
    fn summarize(&self) -> String { self.title.clone() }
}

fn main() {
    assert_eq!(Article { title: String::from("Rust") }.summarize(), "Rust");
}
