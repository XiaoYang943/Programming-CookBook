fn str_len(word: &&str) -> usize {
    word.len()
}

fn main() {
    let mut words = vec!["rust", "a", "closure", "fn"];

    words.sort_by_key(str_len);

    println!("{:?}", words);
    assert_eq!(words, vec!["a", "fn", "rust", "closure"]);
}