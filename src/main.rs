mod editor;

use editor::Editor;

fn main() {
    let editor = Editor::default();
    editor.run();
    // Editor::run(&editor);
}
