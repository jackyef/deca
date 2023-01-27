use std::{fs, io::Write};

use crate::{Position, Row};

#[derive(Default)]
pub struct Document {
    rows: Vec<Row>,
    dirty: bool,
    pub file_name: Option<String>,
}

impl Document {
    pub fn open(filename: &str) -> Result<Self, std::io::Error> {
        let contents = fs::read_to_string(filename)?;
        let mut rows = Vec::new();

        for value in contents.lines() {
            rows.push(Row::from(value));
        }

        Ok(Self {
            rows,
            file_name: Some(filename.to_string()),
            dirty: false,
        })
    }

    pub fn row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub fn len(&self) -> usize {
        self.rows.len()
    }

    pub fn insert(&mut self, at: &Position, c: char) {
        if at.y > self.len() {
            return;
        }

        self.dirty = true;

        if c == '\n' {
            self.insert_newline(at);
            return;
        }

        if at.y == self.len() {
            let mut row = Row::default();
            row.insert(0, c);
            self.rows.push(row);
        } else {
            let row = self.rows.get_mut(at.y).unwrap();
            row.insert(at.x, c);
        }
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn delete(&mut self, at: &Position) {
        let len = self.len();

        if at.y >= len {
            return;
        }

        self.dirty = true;

        let is_at_end_of_row = at.x == self.rows[at.y].len();

        if is_at_end_of_row && at.y + 1 < len {
            let next_row = self.rows.remove(at.y + 1);
            let row = &mut self.rows[at.y];
            row.append(&next_row);
        } else {
            let row = &mut self.rows[at.y];

            row.delete(at.x);
        }
    }

    fn insert_newline(&mut self, at: &Position) {
        let len = self.rows.len();
        if at.y > len {
            return;
        }

        if at.y == len {
            self.rows.push(Row::default());
        } else {
            #[allow(clippy::indexing_slicing)]
            let new_row = self.rows[at.y].split(at.x);
            #[allow(clippy::integer_arithmetic)]
            self.rows.insert(at.y + 1, new_row);
        }
    }

    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(file_name) = &self.file_name {
            let mut file = fs::File::create(file_name)?;

            for row in &self.rows {
                file.write_all(row.as_bytes())?;
                file.write_all(b"\n")?;
            }
            self.dirty = false;
        }

        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }
}
