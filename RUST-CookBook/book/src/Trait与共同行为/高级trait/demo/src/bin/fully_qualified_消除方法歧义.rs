trait First { fn name() -> &'static str; }
trait Second { fn name() -> &'static str; }
struct Type;
impl First for Type { fn name() -> &'static str { "first" } }
impl Second for Type { fn name() -> &'static str { "second" } }

fn main() {
    assert_eq!(<Type as First>::name(), "first");
    assert_eq!(<Type as Second>::name(), "second");
}
