use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


use std::thread;
use std::time::Duration;
fn main() {

    let mut file = File::open("test.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);


    // file.read_to_string(&mut s);
    // println!("{}",s);
    for i in s.split_whitespace(){
        println!("{}\n\n\n\n",i);
    thread::sleep(Duration::from_secs(2));

    }


    // thread::sleep(Duration::from_secs(1));
}
