struct Counter(u32);

impl Counter {
    fn increment(&mut self) {
        self.0 += 1;
    }
}

fn main() {
    let mut counter = Counter(0);
    counter.increment();
    assert_eq!(counter.0, 1);
}
