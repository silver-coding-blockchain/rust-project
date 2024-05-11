use std::{fs, io};
fn main() {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        let prefix = match file_type {
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        };
        println!("{prefix} {}", entry.path().display());
    }
}