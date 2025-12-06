use std::str::Split;

pub trait SplitLines {
    fn split_lines(&self) -> Split<'_, &str>;
}

impl SplitLines for str {
    fn split_lines(&self) -> Split<'_, &str> {
        self.split("\r\n")
    }
}
