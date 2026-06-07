struct UserId(u64);
struct OrderId(u64);

fn load_user(id: UserId) -> UserId {
    id
}

fn main() {
    let order_id = OrderId(42);
    let _user = load_user(order_id);
}
