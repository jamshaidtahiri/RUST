use std::io;

fn main(){
//let mut number=10;
	let mut number=String::new();
	println!("Please enter the number");
	io::stdin().read_line(& mut number).expect("incorrect entry");
	let  mut number:i32=number.trim().parse().unwrap();
    print!("{}",factorial(4));
}
fn factorial(input:i32)->i32{
if input == 0{
    1
}
else {
    input*factorial(input-1)
}
}