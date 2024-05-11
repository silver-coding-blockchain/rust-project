#![allow(unused)]
use std::{fs, io};

pub fn main() {
    let mut entries = fs::read_dir(".")
        .expect("That dir cannot open");
    for entry in entries{
        let file_type=entry.file_type()?;
        let prefix=match file_type{
            t if t.is_dir() => "D",
            t if t.is_file() => "F",
            t if t.is_symlink() => "L",
            _ => "?",
        }
    }    

}