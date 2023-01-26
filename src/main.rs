#![warn(clippy::all, clippy::pedantic)]
mod editor;
mod terminal;
mod document;
mod row;

use editor::Editor;

// Re-exporting Terminal so we can do `use crate::Terminal` instead of `use crate::terminal::Terminal`
pub use terminal::Terminal;
pub use editor::Position;
pub use document::Document;
pub use row::Row;

fn main() {
    Editor::default().run();
}
