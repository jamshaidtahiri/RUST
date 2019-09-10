extern crate colorful;

use colorful::Color;
use colorful::Colorful;
use std::io;
use std::thread;
use std::time::Duration;
extern crate timer;
extern crate chrono;
use std::sync::mpsc::channel;
fn main(){
let timer = timer::Timer::new();
let (tx, rx) = channel();

let _guard = timer.schedule_with_delay(chrono::Duration::seconds(10), move || {
  // This closure is executed on the scheduler thread,
  // so we want to move it away asap.








   
        
    




	let output="WELCOME TO ENTRY TEST 2019";
	println!("{}",output.color(Color::Yellow).bold());
	
	let mut input=String::new();
	println!("Please enter your percentage");
	io::stdin().read_line(&mut input).expect("incorrect entry");
	let input:f32=input.trim().parse().unwrap();
	loop {
	if input >75.0{
	println!("You are eligible for the test");
	let comp="Good luck!!";
	println!("{}",comp.color(Color::Pink3).bold());
}
	else {
	println!("We are sorry you can't apply");
	break
}



	let mut answer_01=String::new();
	println!("Q:1\nAre variables mutable in RUST?\na:yes 	b:no");
	io::stdin().read_line(& mut answer_01).expect("incorrect entry!");
	answer_01=answer_01.trim().to_string();

	let options=["a","b","c","d"];
	let mut points=0;
	if answer_01==options[0]{
	let cp="Correct Answer!!";
	println!("{}",cp.color(Color::Green).bold());
	points+=1
}
	else{
	let wa="Wrong Answer!!";
	println!("{}",wa.color(Color::Red));}
	

	let mut answer_02=String::new();
	println!("Q:2\nThe default integer type in RUST is?\na:i32 b:u8");
	io::stdin().read_line(& mut answer_02).expect("incorrect answer");
	answer_02=answer_02.trim().to_string();
	let op=["a","b","c","d"];
	if answer_02==op[0]{
	let co="Correct Answer!!";
	println!("{}",co.color(Color::Green).bold());
	points+=1	
}
	else {
	let wans="Wrong answer!!";
	println!("{}",wans.color(Color::Red));}

	let mut answer_03=String::new();
	println!("Q:3\nWhich one of them is an unsigned integer?\na:i32  b:u32");
	io::stdin().read_line(&mut answer_03).expect("Incorrect entry!");
	answer_03=answer_03.trim().to_string();
	let ot=["a","b"];
	if answer_03==ot[1] {
	let ca="Correct Answer!!";
	println!("{}",ca.color(Color::Green).bold());
	points+=1
}
	else {
	let wr="Wrong answer!!";
	println!("{}",wr.color(Color::Red));
}
	println!(" Your score is {}",points);
	break
	if points==3 {
	let result="Congratulations you have passed the Entry Test!!";
	println!("{}",result.color(Color::Yellow).bold());}
 }









  let _ignored = tx.send("done"); // Avoid unwrapping here.
});

let x = rx.recv().unwrap();
println!("This code has been executed after 3 seconds {}",x);
}