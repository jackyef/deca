#![warn(clippy::all, clippy::pedantic)]
mod editor;
use editor::Editor;

mod terminal;
// Re-exporting Terminal so we can do `use crate::Terminal` instead of `use crate::terminal::Terminal`
pub use terminal::Terminal;
pub use editor::Position;

fn main() {
    Editor::default().run();
}
