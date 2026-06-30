fn main() {
    let inputs = ["10", "invalid", "20", "3"];
    let even_numbers: Vec<i32> = inputs
        .iter()
        .filter_map(|text| text.parse::<i32>().ok())
        .filter(|number| number % 2 == 0)
        .collect();

    assert_eq!(even_numbers, [10, 20]);
}
