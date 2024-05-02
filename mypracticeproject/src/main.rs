use std::env;
use std::fs;
#[derive(Debug,PartialEq,Copy,Clone)]
enum ShirtColor{
    Red,
    Blue,
}
struct Inventory{
    shirts:Vec<ShirtColor>,
}
impl Inventory{
    fn giveway(&self, user_preference:Option<ShirtColor>)->ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    fn most_stocked(&self)->ShirtColor{
        let mut num_red=0;
        let mut num_blue=0;
        for color in &self.shirts{
            match color{
                ShirtColor::Red=>num_red+=1,
                ShirtColor::Blue=>num_blue+=1,
            }
        }
        if num_red>num_blue{
            ShirtColor::Red
        } else{
            ShirtColor::Blue
        }
    }
}
fn pro1(){
    let file_path="poem.txt";
    println!("In file {}", file_path);
    let contents=fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    println!("With text:\n{contents}");
}
fn pro2(){
    let store=Inventory{
        shirts:vec![ShirtColor::Blue,ShirtColor::Red, ShirtColor::Blue],
    };
    let user_pref1=Some(ShirtColor::Red);
    let giveway1=store.giveway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}", user_pref1, giveway1
    );
    let user_pref2=None;
    let giveway2=store.giveway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}", user_pref2, giveway2
    );    
}
fn pro3(){
    let list=vec![1,2,3];
    println!("Before defining closure:{:?}",list);
    let only_borrows=|| println!("From closure:{:?}", list);
    println!("Before calling closure:{:?}", list);
    only_borrows();
    println!("After calling closure:{:?}", list);
}
#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}
fn pro4(){
    let mut list=[
        Rectangle{width:10, height:1},
        Rectangle{width:3, height:5},
        Rectangle{width:7, height:12},
    ];
    list.sort_by_key(|r| r.width);
    println!("{:#?}",list);
}
fn main() {
    println!("Hello, world!");
    //pro1();
    //pro2();
    //pro3();
    pro4();
}
