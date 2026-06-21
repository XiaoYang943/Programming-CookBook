use std::borrow::Cow;

#[derive(Debug)]
struct Text<'a> {
    // Cow 可以保存借用的 str，也可以保存拥有的 String。
    value: Cow<'a, str>,
}

impl<'a> Text<'a> {
    fn from_borrowed(value: &'a str) -> Self {
        Self {
            // Borrowed 不分配新字符串，只保存引用。
            value: Cow::Borrowed(value),
        }
    }

    fn from_owned(value: String) -> Self {
        Self {
            // Owned 接收 String 的所有权。
            value: Cow::Owned(value),
        }
    }

    fn ensure_suffix(&mut self, suffix: &str) {
        if !self.value.ends_with(suffix) {
            // to_mut 会在需要修改时把 Borrowed 克隆成 Owned。
            self.value.to_mut().push_str(suffix);
        }
    }
}

fn main() {
    let borrowed = "hello";
    let mut text1 = Text::from_borrowed(borrowed);
    let mut text2 = Text::from_owned(String::from("rust"));

    text1.ensure_suffix("!");
    text2.ensure_suffix("!");

    println!("text1 = {:?}", text1);
    println!("text2 = {:?}", text2);
}
