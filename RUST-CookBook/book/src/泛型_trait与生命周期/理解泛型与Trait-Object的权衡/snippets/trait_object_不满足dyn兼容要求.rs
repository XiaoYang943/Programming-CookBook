trait CloneSelf {
    fn clone_self(&self) -> Self;
}

fn main() {
    let _value: &dyn CloneSelf;
}
