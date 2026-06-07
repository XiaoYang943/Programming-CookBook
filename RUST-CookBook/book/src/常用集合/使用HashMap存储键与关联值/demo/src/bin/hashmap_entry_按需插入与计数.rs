use std::collections::HashMap;

fn main() {
    let mut counts = HashMap::new();
    for word in "hello world hello".split_whitespace() {
        *counts.entry(word).or_insert(0) += 1;
    }
    assert_eq!(counts["hello"], 2);
}
