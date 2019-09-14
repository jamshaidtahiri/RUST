extern crate fast_inv_sqrt;
use fast_inv_sqrt::InvSqrt32;
use fast_inv_sqrt::InvSqrt64;
use std::io;
fn main() {
	println!("	SIMPLE CALCULATOR");
	
	println!("Follow the the following numeric values in order to get your  desired operation be performed");
	println!("SELECT\n1: ➕  2: ➖   3: ➗   4: ✖  5: ❗  6: ^2  7:^3   8:a^n  9:1/n  10:sqrt(inv)    11:sqrt  12:log()  13:% ");

	let  mut inp_1=String::new();
	println!("Enter the option");
	io::stdin().read_line(& mut inp_1).expect("incorrect entry");
	let  inp_1:u8=inp_1.trim().parse().unwrap();
let options=[1,2,3,4,5,6,7,8,9,10,11,12,13];
if inp_1==options[0]{

let  mut a=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let  a:f64=a.trim().parse().unwrap();

let  mut b=String::new();
	println!("Enter the second number");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let b:f64=b.trim().parse().unwrap();
let addition=a+b;
println!("the answer is {}",addition);
}
else if inp_1==options[1]{
let  mut a=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let  a:f64=a.trim().parse().unwrap();

let  mut b=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let  b:f64=b.trim().parse().unwrap();
let subtraction=a-b;
	println!("the answer is {}",subtraction);
}
else if inp_1==options[2]{
let  mut a=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let  a:f64=a.trim().parse().unwrap();

let  mut b=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let  b:f64=b.trim().parse().unwrap();
let division=a/b;
println!("the answer is {}",division);
}
else if inp_1==options[3]{

let  mut a=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let  a:i64=a.trim().parse().unwrap();


let  mut b=String::new();
	println!("Enter the first number");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let  b:i64=b.trim().parse().unwrap();
let multiplication=a*b;

println!("the answer is {}",multiplication);
}
else if inp_1==options[4]{

	let mut number=String::new();
	println!("Please enter the number");
	io::stdin().read_line(& mut number).expect("incorrect entry");
	let  mut number:u8=number.trim().parse().unwrap();
    let mut x=1;
    for i in 1..number{
         x=x+i*x;
    }
println!("{}",x);
}

else if inp_1==options[5]{

let  mut a=String::new();
	println!("Enter the  number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let  a:i64=a.trim().parse().unwrap();


let square=a*a;
println!("the answer is {}",square);
}

else if inp_1==options[6]{
	
let  mut a=String::new();
	println!("Enter the number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let mut a:i64=a.trim().parse().unwrap();



let cube=a*a*a;
println!("the answer is {}",cube);
}
else if inp_1==options[7]{

let  mut a=String::new();
	println!("Enter the  first number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let mut a:u32=a.trim().parse().unwrap();


let  mut b=String::new();
	println!("Enter the second number");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let  b:u32=b.trim().parse().unwrap();
let power=u32::pow(a,b);
println!("the ans is {}",power);
}

else if  inp_1==options[8]{

let  mut a=String::new();
	println!("Enter the  number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let a:f64=a.trim().parse().unwrap();
let inverse=1.0/a;
println!("the answer is {}",inverse);
}
else if inp_1==options[9]{

let mut a=String::new();
	println!("Enter the  floating point number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let a:f64=a.trim().parse().unwrap();
println!("the answer is {}",a.inv_sqrt32());

let mut b=String::new();
	println!("Enter the integer");
	io::stdin().read_line(& mut b).expect("incorrect entry");
	let b:i8=b.trim().parse().unwrap();
println!("the answer is {}",b.inv_sqrt32());
}

else if inp_1==options[10]{

let mut a=String::new();
	println!("Enter the  floating point number");
	io::stdin().read_line(& mut a).expect("incorrect entry");
	let a:i64=a.trim().parse().unwrap();
    println!("the answer is {}",((a as f64).sqrt()));
//assert_eq!((a*a).sqrt(),a);
}

}
