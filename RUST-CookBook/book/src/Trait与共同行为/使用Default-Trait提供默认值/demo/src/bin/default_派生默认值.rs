#[derive(Default)]
struct Config { retries: u32 }

fn main() {
    assert_eq!(Config::default().retries, 0);
}
