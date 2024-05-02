use std::env;
use std::fs;

fn pro1(){
    let file_path="poem.txt";
    println!("In file {}", file_path);
    let contents=fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
fn main() {
    println!("Hello, world!");
    pro1();
}
