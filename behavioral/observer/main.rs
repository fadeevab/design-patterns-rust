use editor::Editor;
use observer::Event;

mod editor;
mod observer;

fn main() {
    let mut editor = Editor::default();

    editor.events().subscribe(Event::Load, |file_path| {
        let log = "/path/to/log/file.txt".to_string();
        println!("Save log to {}: Load file {}", log, file_path);
    });

    editor.events().subscribe(Event::Save, save_listener);

    editor.load("test1.txt".into());
    editor.load("test2.txt".into());
    editor.save();

    editor.events().unsubscribe(Event::Save, save_listener);
    editor.save();
}

fn save_listener(file_path: String) {
    let email = "admin@example.com".to_string();
    println!("Email to {}: Save file {}", email, file_path);
}
