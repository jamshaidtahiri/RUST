#[macro_use]
extern crate text_io;
use std::io;
fn main() {
  
let mut anyvec : Vec<(String,i32)> = vec![];
let num0finput : i32 = read!();

for _ in 0..num0finput{
    let string = read!();
    let number = read!();

    let tup = (string,number);
    anyvec.push(tup);
}
print!("{:?}",anyvec );

let min = smallest(&anyvec);

while min == smallest(&anyvec){
    for i  in 0..(anyvec.len()){
            let  n = anyvec[i as usize].1;
        if min == n{
            anyvec.remove(i);
            break;
        }
    }
}
    let min = smallest(&anyvec);
    println!("{}",min);

    for i  in 0..(anyvec.len()){
        if min == anyvec[i as usize].1{
            println!("{}",anyvec[i as usize].0 );
        }
    }  
}

fn smallest(a : &Vec<(String,i32)>) -> i32{
    let mut smallest =a[0 as usize].1;
    for i in 0..a.len(){
        let  n = a[i as usize].1;
        if n <smallest{
            smallest = n;
        }
    }
    smallest
}
