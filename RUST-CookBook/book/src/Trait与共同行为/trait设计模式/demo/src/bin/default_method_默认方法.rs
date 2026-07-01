#[derive(Debug)]
enum ValueError {
    Negative,
    TooLarge,
}

trait Validate {
    // 关联错误类型：不同实现可以返回不同的错误类型。
    type Error;

    // 必需方法：实现者只需要说明“如何收集错误”。
    fn errors(&self) -> Vec<Self::Error>;

    fn is_valid(&self) -> bool {
        // 默认方法：只要没有错误，就认为有效。
        self.errors().is_empty()
    }

    fn check(&self) -> Result<(), Self::Error> {
        // 默认方法：复用 errors，只返回第一个错误。
        match self.errors().into_iter().next() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

struct LimitedValue {
    value: i32,
}

impl Validate for LimitedValue {
    type Error = ValueError;

    fn errors(&self) -> Vec<Self::Error> {
        let mut errors = Vec::new();

        if self.value < 0 {
            errors.push(ValueError::Negative);
        }

        if self.value > 100 {
            errors.push(ValueError::TooLarge);
        }

        errors
    }
}

fn main() {
    let value = LimitedValue { value: 120 };

    // LimitedValue 只实现了 errors，却自动获得 is_valid 和 check。
    println!("is_valid = {}", value.is_valid());
    println!("check = {:?}", value.check());
}
