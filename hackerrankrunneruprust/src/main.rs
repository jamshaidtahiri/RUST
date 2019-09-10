
use std::io;
fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let mut input1 : i32 = input1.trim().parse().unwrap();
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let mut input2 : i32 = input2.trim().parse().unwrap();
    
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let mut input3 : i32 = input3.trim().parse().unwrap();
    
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4);
    let mut input4 : i32 = input4.trim().parse().unwrap();
    
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5);
    let mut input5 : i32 = input5.trim().parse().unwrap();

    let mut array = [input1,input2,input3,input4,input5];

    array.sort();
    let x = array.len();

    println!("{:?}",array[x-1]);

 
}
