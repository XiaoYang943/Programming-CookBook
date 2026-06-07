fn main() {
    let value = 42;
    let pointer: *const i32 = &value;

    unsafe {
        assert_eq!(*pointer, 42);
    }
}
