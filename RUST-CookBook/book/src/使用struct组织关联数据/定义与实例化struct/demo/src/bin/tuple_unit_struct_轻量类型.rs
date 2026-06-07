struct Color(u8, u8, u8);
struct Marker;

fn main() {
    let black = Color(0, 0, 0);
    let _marker = Marker;

    assert_eq!((black.0, black.1, black.2), (0, 0, 0));
}
