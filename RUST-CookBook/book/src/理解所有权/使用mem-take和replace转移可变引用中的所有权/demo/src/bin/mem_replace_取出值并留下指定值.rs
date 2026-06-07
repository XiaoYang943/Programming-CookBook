use std::mem;

fn main() {
    let mut state = String::from("pending");
    let previous = mem::replace(&mut state, String::from("complete"));

    assert_eq!(previous, "pending");
    assert_eq!(state, "complete");
}
