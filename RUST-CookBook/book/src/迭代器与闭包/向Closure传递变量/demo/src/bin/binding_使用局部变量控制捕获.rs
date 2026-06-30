struct Config {
    minimum: i32,
    maximum: i32,
}

fn main() {
    let config = Config {
        minimum: 10,
        maximum: 20,
    };
    let minimum = config.minimum;
    let accepts = |value| value >= minimum;

    assert!(accepts(12));
    assert_eq!(config.maximum, 20);
}
