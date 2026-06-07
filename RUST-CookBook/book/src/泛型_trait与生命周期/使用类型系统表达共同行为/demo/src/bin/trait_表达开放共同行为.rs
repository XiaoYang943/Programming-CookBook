trait Area { fn area(&self) -> u32; }
struct Square(u32);
impl Area for Square { fn area(&self) -> u32 { self.0 * self.0 } }

fn main() {
    assert_eq!(Square(4).area(), 16);
}
