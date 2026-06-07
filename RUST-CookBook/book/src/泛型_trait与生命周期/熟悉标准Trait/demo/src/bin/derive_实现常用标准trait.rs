#[derive(Debug, Clone, Default, PartialEq)]
struct Config {
    retries: u32,
}

fn main() {
    let config = Config::default();
    assert_eq!(config.clone(), config);
    assert!(format!("{config:?}").contains("retries"));
}
