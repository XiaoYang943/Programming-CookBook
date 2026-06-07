struct Account {
    name: String,
    visits: u32,
}

fn main() {
    let mut account = Account {
        name: String::from("Alice"),
        visits: 0,
    };
    let name = &account.name;
    let visits = &mut account.visits;

    *visits += 1;
    assert_eq!(name, "Alice");
    assert_eq!(*visits, 1);
}
