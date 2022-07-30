mod fs;

use fs::{Component, File, Folder};

fn main() {
    let file1 = File::new("File 1".into());
    let file2 = File::new("File 2".into());
    let file3 = File::new("File 3".into());

    let mut folder1 = Folder::new("Folder 1".into());
    folder1.add(file1);

    let mut folder2 = Folder::new("Folder 2".into());
    folder2.add(file2);
    folder2.add(file3);
    folder2.add(folder1);

    folder2.search(&"rose".into());
}
