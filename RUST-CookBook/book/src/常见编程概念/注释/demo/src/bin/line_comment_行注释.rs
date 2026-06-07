fn main() {
    // 使用整数分表示金额，避免浮点数舍入误差。
    let price_in_cents = 1_299;

    assert_eq!(price_in_cents, 1_299);
}
