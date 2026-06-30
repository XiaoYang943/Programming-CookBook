fn call_fn(action: impl Fn() -> usize) -> usize {
    action() + action()
}

fn call_fn_mut(mut action: impl FnMut()) {
    action();
    action();
}

fn call_fn_once(action: impl FnOnce() -> String) -> String {
    action()
}

fn main() {
    let text = String::from("rust");
    assert_eq!(call_fn(|| text.len()), 8);

    let mut count = 0;
    call_fn_mut(|| count += 1);
    assert_eq!(count, 2);

    let owned = String::from("moved");
    assert_eq!(call_fn_once(|| owned), "moved");
}
