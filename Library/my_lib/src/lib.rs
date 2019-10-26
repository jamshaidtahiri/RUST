 pub mod math {
    
use std::io;
pub fn factorial(){
	let mut number=String::new();
	println!("Please enter the number");
	io::stdin().read_line(& mut number).expect("incorrect entry");
	let  mut number:u64=number.trim().parse().unwrap();
    println!("{}",fact(number));
}
fn fact(input:u64)->u64{
if input == 0{
    1
}
else {
    input*fact(input-1)
}
}
}


