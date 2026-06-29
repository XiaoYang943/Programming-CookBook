fn main() {
    let mut words = vec!["rust", "a", "closure", "fn"];

    words.sort_by_key(|word| word.len());   // 按字符串长度排序

    println!("{:?}", words);
    assert_eq!(words, vec!["a", "fn", "rust", "closure"]);
}