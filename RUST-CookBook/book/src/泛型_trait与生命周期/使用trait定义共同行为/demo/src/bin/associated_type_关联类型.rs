trait Parser {
    // 关联类型：每个 Parser 实现者自己决定 parse 之后产出什么类型。
    type Output;

    fn parse(&self, input: &str) -> Self::Output;
}

struct NumberParser;

impl Parser for NumberParser {
    // NumberParser 的 parse 结果是 i32。
    type Output = i32;

    fn parse(&self, input: &str) -> Self::Output {
        input
            .trim()
            .parse()
            .expect("NumberParser 只能解析整数文本")
    }
}

struct BoolParser;

impl Parser for BoolParser {
    // BoolParser 的 parse 结果是 bool。
    type Output = bool;

    fn parse(&self, input: &str) -> Self::Output {
        matches!(input.trim(), "true" | "yes" | "1")
    }
}

struct WordsParser;

impl Parser for WordsParser {
    // WordsParser 的 parse 结果是 Vec<String>。
    type Output = Vec<String>;

    fn parse(&self, input: &str) -> Self::Output {
        input
            .split_whitespace()
            .map(String::from)
            .collect()
    }
}

fn main() {
    let number_parser = NumberParser;
    let bool_parser = BoolParser;
    let words_parser = WordsParser;

    let number = number_parser.parse("42");
    let flag = bool_parser.parse("yes");
    let words = words_parser.parse("rust trait parser");

    // 同一个 trait 方法 parse，因为实现者不同，返回类型也不同。
    println!("number + 1 = {}", number + 1);
    println!("flag = {}", flag);
    println!("words = {:?}", words);
}
