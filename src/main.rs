
#[path = "Editor.rs"]
mod editor;

use editor::Editor;
fn main() {
    Editor::run().unwrap();
}
