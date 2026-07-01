struct Meters(u32);
struct Seconds(u32);

fn speed(distance: Meters, time: Seconds) -> u32 {
    distance.0 / time.0
}

fn main() {
    assert_eq!(speed(Meters(100), Seconds(10)), 10);
}
