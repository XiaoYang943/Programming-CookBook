fn search<'a>(query: &str, contents: &'a str, ignore_case: bool) -> Vec<&'a str> {
    let normalized_query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| {
            if ignore_case {
                line.to_lowercase().contains(&normalized_query)
            } else {
                line.contains(query)
            }
        })
        .collect()
}

fn main() {
    let contents = "Rust\nrustacean\nIterator";

    assert_eq!(search("rust", contents, false), ["rustacean"]);
    assert_eq!(search("rust", contents, true), ["Rust", "rustacean"]);
}
