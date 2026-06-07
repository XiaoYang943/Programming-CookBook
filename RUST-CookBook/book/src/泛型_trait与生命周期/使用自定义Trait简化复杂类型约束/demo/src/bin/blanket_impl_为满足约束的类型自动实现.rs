trait Double {
    fn double(self) -> Self;
}

impl<T> Double for T
where
    T: Copy + std::ops::Add<Output = T>,
{
    fn double(self) -> Self { self + self }
}

fn main() {
    assert_eq!(21.double(), 42);
}
