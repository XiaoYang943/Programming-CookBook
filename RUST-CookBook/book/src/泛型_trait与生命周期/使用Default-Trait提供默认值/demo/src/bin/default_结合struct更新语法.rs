#[derive(Default)]
struct Config { retries: u32, verbose: bool }

fn main() {
    let config = Config { retries: 3, ..Default::default() };
    assert_eq!(config.retries, 3);
    assert!(!config.verbose);
}
