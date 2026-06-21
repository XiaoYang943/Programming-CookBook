trait Parser<T> {
    // 这里没有 type Output。
    // 输出类型 T 写在 trait 名字上，由实现或调用场景决定。
    fn parse(&self, input: &str) -> T;
}

struct FlexibleParser;

impl Parser<i32> for FlexibleParser {
    // 同一个 FlexibleParser，可以实现“解析成 i32”的能力。
    fn parse(&self, input: &str) -> i32 {
        input.trim().parse().expect("只能解析整数文本")
    }
}

impl Parser<bool> for FlexibleParser {
    // 同一个 FlexibleParser，也可以实现“解析成 bool”的能力。
    fn parse(&self, input: &str) -> bool {
        matches!(input.trim(), "true" | "yes" | "1")
    }
}

impl Parser<Vec<String>> for FlexibleParser {
    // 同一个 FlexibleParser，还可以实现“解析成 Vec<String>”的能力。
    fn parse(&self, input: &str) -> Vec<String> {
        input.split_whitespace().map(String::from).collect()
    }
}

fn main() {
    let parser = FlexibleParser;

    // 因为 FlexibleParser 同时实现了 Parser<i32>、Parser<bool>、Parser<Vec<String>>，
    // 所以调用 parse 时，需要通过类型标注告诉编译器要选择哪一个实现。
    let number: i32 = parser.parse("42");
    let flag: bool = parser.parse("yes");
    let words: Vec<String> = parser.parse("rust trait parser");

    println!("number + 1 = {}", number + 1);
    println!("flag = {}", flag);
    println!("words = {:?}", words);

    // 如果没有类型标注，编译器不知道该选哪个 Parser<T> 实现：
    // let value = parser.parse("42");

    // 也可以使用完全限定语法明确指定 T。
    let explicit = <FlexibleParser as Parser<i32>>::parse(&parser, "100");
    println!("explicit + 1 = {}", explicit + 1);
}
