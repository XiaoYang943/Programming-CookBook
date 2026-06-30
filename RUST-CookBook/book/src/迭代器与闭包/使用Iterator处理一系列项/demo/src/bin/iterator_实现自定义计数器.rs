struct Counter {
    current: u32,
    end: u32,
}

impl Counter {
    fn new(end: u32) -> Self {
        Self { current: 0, end }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        (self.current <= self.end).then_some(self.current)
    }
}

fn main() {
    assert_eq!(Counter::new(4).collect::<Vec<_>>(), [1, 2, 3, 4]);
    assert_eq!(Counter::new(4).map(|value| value * value).sum::<u32>(), 30);
}
