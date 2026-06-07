fn main() {
    let mut visits = 0;

    'outer: for row in 0..3 {
        for column in 0..3 {
            if row == 1 && column == 1 {
                break 'outer;
            }
            visits += 1;
        }
    }

    assert_eq!(visits, 4);
}
