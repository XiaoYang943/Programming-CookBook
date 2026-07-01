#[derive(Debug, PartialEq)]
struct Request {
    timeout: u64,
    retries: u32,
}

struct RequestBuilder {
    timeout: u64,
    retries: u32,
}

impl RequestBuilder {
    fn new() -> Self {
        Self { timeout: 30, retries: 0 }
    }

    fn retries(mut self, retries: u32) -> Self {
        self.retries = retries;
        self
    }

    fn build(self) -> Request {
        Request { timeout: self.timeout, retries: self.retries }
    }
}

fn main() {
    let request = RequestBuilder::new().retries(3).build();
    assert_eq!(request, Request { timeout: 30, retries: 3 });
}
