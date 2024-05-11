#![allow(unused)]

use std::{cmp::Ordering, thread::Result};
//mod filesystem;
mod networking_server;
mod networking_client;
fn main(){
    let mut arg=std::env::args();
    //let _result=filesystem::main();
    let args:Vec<String> = arg.into_iter().map(|str| str).collect();
    if args[1]=="server" {
        networking_server::main();
    } else{
        networking_client::main();
    }
}