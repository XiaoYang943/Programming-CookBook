use std::fmt;

struct DisplayVec(Vec<i32>);

impl fmt::Display for DisplayVec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

fn main() {
    assert_eq!(DisplayVec(vec![1, 2]).to_string(), "[1, 2]");
}
