struct Writer { finished: bool }

impl Writer {
    fn finish(mut self) -> Result<(), &'static str> {
        self.finished = true;
        Ok(())
    }
}

fn main() {
    assert!(Writer { finished: false }.finish().is_ok());
}
