 pub mod math {
    
use std::io;
pub fn factorial(){
	let mut number=String::new();
	println!("Please enter the number");
	io::stdin().read_line(& mut number).expect("incorrect entry");
	let  mut number:i64=number.trim().parse().unwrap();
    println!("{}",fact(number));
}
fn fact(input:i64)->i64{
if input == 0{
    1
}
else {
    input*fact(input-1)
}
}
}