enum Outcome<T, E> {
    Success(T),
    Failure(E),
}

fn main() {
    let success: Outcome<i32, &str> = Outcome::Success(42);
    let failure: Outcome<i32, &str> = Outcome::Failure("error");
    assert!(matches!(success, Outcome::Success(42)));
    assert!(matches!(failure, Outcome::Failure("error")));
}
