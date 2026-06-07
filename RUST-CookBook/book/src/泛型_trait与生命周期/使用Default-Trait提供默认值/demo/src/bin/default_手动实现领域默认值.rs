struct Config { retries: u32 }

impl Default for Config {
    fn default() -> Self { Self { retries: 3 } }
}

fn main() {
    assert_eq!(Config::default().retries, 3);
}
