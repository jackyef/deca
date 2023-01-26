use std::cmp;

pub struct Row {
    string: String,
}

impl From<&str> for Row {
    fn from(value: &str) -> Self {
        Self {
            string: String::from(value),
        }
    }
}

impl Row {
    pub fn render(&self, start: usize, end: usize) -> String {
        let end = cmp::min(end, self.string.len());
        let start = cmp::min(start, end);
        self.string.get(start..end).unwrap_or_default().to_string()
    }
}
