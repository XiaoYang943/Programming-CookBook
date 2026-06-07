use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert("Blue", 10);
    assert_eq!(scores.get("Blue"), Some(&10));
}
