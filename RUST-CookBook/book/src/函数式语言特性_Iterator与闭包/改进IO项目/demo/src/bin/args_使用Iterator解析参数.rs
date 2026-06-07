#[derive(Debug, PartialEq)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        let _program = args.next();
        let query = args.next().ok_or("缺少查询文本")?;
        let file_path = args.next().ok_or("缺少文件路径")?;
        Ok(Self { query, file_path })
    }
}

fn main() {
    let args = ["search", "rust", "notes.txt"]
        .map(String::from)
        .into_iter();
    let config = Config::build(args).unwrap();

    assert_eq!(
        config,
        Config {
            query: String::from("rust"),
            file_path: String::from("notes.txt"),
        }
    );
}
