#[derive(Debug, Clone, Copy)]
struct Pair {
    left: i32,
    right: i32,
}

trait MapPair {
    fn map_pair(self, f: impl Fn(i32) -> i32) -> Self;
}

impl MapPair for Pair {
    fn map_pair(self, f: impl Fn(i32) -> i32) -> Self {
        Self {
            left: f(self.left),
            right: f(self.right),
        }
    }
}

trait Shift: MapPair + Sized {
    fn shift(self, offset: i32) -> Self {
        // Shift 不关心具体类型有多少字段，只依赖 MapPair 这项基础能力。
        self.map_pair(|value| value + offset)
    }
}

// blanket impl：任何实现了 MapPair 的类型，都会自动实现 Shift。
impl<T> Shift for T where T: MapPair + Sized {}

fn main() {
    let pair = Pair { left: 1, right: 4 };

    // Pair 没有单独写 impl Shift for Pair，但因为它实现了 MapPair，所以自动拥有 shift。
    let shifted = pair.shift(10);

    println!("shifted = {:?}", shifted);
}
