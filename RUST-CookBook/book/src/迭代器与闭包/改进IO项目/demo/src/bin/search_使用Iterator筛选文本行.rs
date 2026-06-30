fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn main() {
    let contents = "Rust is fast.\nRust is safe.\nIterators are lazy.";

    assert_eq!(search("Rust", contents), ["Rust is fast.", "Rust is safe."]);
}
