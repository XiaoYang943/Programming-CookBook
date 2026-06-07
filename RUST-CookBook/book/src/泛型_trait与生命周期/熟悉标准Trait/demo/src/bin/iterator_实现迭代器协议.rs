struct Counter(u32);

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        (self.0 <= 3).then_some(self.0)
    }
}

fn main() {
    assert_eq!(Counter(0).sum::<u32>(), 6);
}
