enum List {
    Cons(Box<List>),
    Nil,
}

fn length(list: &List) -> usize {
    match list {
        List::Cons(next) => 1 + length(next),
        List::Nil => 0,
    }
}

fn main() {
    let list = List::Cons(Box::new(List::Cons(Box::new(List::Nil))));
    assert_eq!(length(&list), 2);
}
