// use std::collections::HashMap;
// fn main() {
    
// let teams  = vec![String::from("Blue"), String::from("Yellow")];
// let initial_scores = vec![10, 50];

// let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
// print!("{:?}",scores);
// }




// use std::io;

// fn main() {

//     let mut initial_length = 0;
//     let final_length = add_length();
    
//     let mut vec : Vec<i32> = Vec::new();

//     while initial_length != final_length {
//         vec.push(add_vector());
//         initial_length += 1;
//     }

//     // println!("{}", final_length);

//     for i in vec.iter().rev() {
//         print!("{} ", i);
//     }    

// }

// fn add_length() -> i32 {
//     let mut user_length = String::new();
//     io::stdin().read_line(&mut user_length)
//         .expect("Failed to read user input");
    
//     let user_length: i32 = match user_length.trim().parse() {
//         Ok(num) => num,
//         Err(_) => 0,
//     };
//     user_length
// }

// fn add_vector() -> i32 {
//     let mut user_vector_inputs = String::new();
//     io::stdin().read_line(&mut user_vector_inputs)
//         .expect("Failed to read user input");

//     let user_vector_inputs: i32 = match user_vector_inputs.trim().parse(){
//         Ok(num) => num,
//         Err(_) => 0,
//     };
//     user_vector_inputs
// }





// use std::{io};
// fn main() {

    
//     let mut new_vector: Vec<i32> = Vec::new();

//     for i in 1..5{
//      let mut my_vector = String::new();
//     io::stdin().read_line(&mut my_vector);

//     // let my_vector: i32 = match my_vector.trim().parse(){
//     //     Ok(num) => num,
//     //     Err(_) => 0
//     // };
//  let my_vector: i32 = my_vector.trim().parse().expect("wrong input");
//      new_vector.push(my_vector);
//     }
     
//     for i in new_vector.iter(){isalnum
        
//     println!("The vector value is: {}",i);
    
//     }
//     }


// use std::io;
// fn main() {
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     // let input = input.trim().parse().unwrap();
    
//     let mut emptyvec = vec![];
//     for i in input.split_whitespace(){
//     let x : i32 = i.parse().unwrap();
// emptyvec.push(x);
//     // println!("{}",i );
//     }
//     for i in emptyvec.iter().rev(){
// print!("{} ",i);
//     }
// }



// use std::io;
// fn main() {
// 	let mut a=String::new();
// 	println!("Enter the String");
// 	io::stdin().read_line(&mut a).expect("incorrect entry");
//     a=a.trim().to_string();
	

// 	let mut b=String::new();
// 	println!("How many copies of String you need");
// 	io::stdin().read_line(&mut b).expect("incorrect entry");
// 	let b:i32=b.trim().parse().unwrap();
//     for i in 0..b{
//         print!("{}",a);
//     }

// }


// fn main(){
//     let name = ["haris","ahmad","saad"];
//     let age = [12,13,14];
//     for i in 0..name.len(){
//     println!("{} : {}",name[i],age[i]);
//     }
// }





// use std::io;
// #[derive(Debug)]
// struct USER {
//     username: String,
//     password: String,
// }
// fn main(){
// let user =USER{
//     username: String::from("jamshaidtahiri"),
//     password: String::from("pass"),
// };
// println!("{:?}",user);


// let user1 = register();
// println!("{:?}",user1);

// }
// fn register () -> USER{
//     let mut username = String::new();
//     let mut password = String::new();
//     io::stdin().read_line(&mut username);
//     io::stdin().read_line(&mut password);
//     username = username.trim().to_string();
//     password = password.trim().to_string();



//     USER{
// username,
// password,
//     }
// }

// use std::io;
// fn main(){
//     // let int = String::new();
//     let mut input =String::new();
//     io::stdin().read_line(&mut input);

//     let mut anyvec = vec![];
//     for i in input.split_whitespace(){
//         let num :i32  = i.parse().unwrap();
//         anyvec.push(num);
//         // println!("{}",i);
//     }
//     println!("{:?}",anyvec);
// }


// use std::io;
// fn main() {re starting in Islamabad this Sunday. The 
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     // let mut input = input.trim();
// let mut anyvec = vec![];
// let mut vector : Vec<i32> = vec![];

//     for i in 0..input.len(){
//         let num = &input[i..i+1];
//         if num == " "{
//             let n = anyvec.join("");
//             let n:i32 = n.parse().unwrap();
//             vector.push(n);
//             for i in 0..anyvec.len(){
//                 anyvec.pop();
//             }            

//         }else {
//             anyvec.push(num);
//         }
        
//     }
// println!("{:?}",vector);
// }







// #[derive(Debug,Clone)]
// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
//  fn main(){
//      let mut user1 = User {
//     email: String::from("jamshaid"),
//     username: String::from("someusername123"),
//     active: true,
//     sign_in_count: 1,
// };
// // user1.email=String::from("ndkxm;mf;fm");
// let user2 =User{
// ..user1.clone()
// };
// println!("user info : {:?}",user1);
// println!("user2 info : {:?}",user2);
//  }







// #[macro_use]
// extern crate text_io;

// #[derive(Debug)]
// struct User {
//     username : String,
//     password : String,    
// }

// fn main() {
//     let username : String = read!();
//     let password: String = read!();

//     print!("{:?}",user(username, password) );
// }

// fn user (username: String,password:String)-> User{
//     User{
//         username,
//         password,
//     }
// }


// #[derive(Debug)]
// enum User {
//     Player {
//     first_name: (String),
//     last_name: (String),
//     }
// }

// impl crate::User {
//     fn full_name(&self) -> String {
//         match self{
//             User::Player{first_name,last_name} => format!("first_name:{}, last_name: {}",first_name,last_name),
//             // _ => "error".to_string(),
//         }
//         // let User::Player{first_name,last_name}
//         // format!("{} {}", self.first_name, self.last_name)
//     }
// }

// fn main() {
//     let player_1 = User::Player {
//         first_name: "Rafael".to_string(),
//         last_name: "Nadal".to_string(),
//     };

//     println!("Player 01: {:#?}", player_1.full_name());
// }








// fn main() {
// 	print!("{}",first_word(&"hello world".to_string()));
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();
// 	print!("{:?}",bytes);

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }



// #[derive(Debug)]

//  struct Student {

//  Name: String,

//  Grade: String,

//  Age: i8,

//  Percentage: f32,
// #[derive(Debug)]

//  struct Student {

//  Name: String,

//  Grade: String,

//  Age: i8,

//  Percentage: f32,

//  }

// impl Student {

// fn new() -> Student {

// Student {Name: String::from("Ghufran"), Grade: String::from("A"), Age: 27, Percentage: 74.9}

// }

// fn PercentageShower(&self) -> f32 {

//        self.Percentage

// }

// }  

// fn main() {

//     let studentdetails =  Student::new();

//     println!("{:#?}",studentdetails); 

//     println!("Percentage is {}",studentdetails.PercentageShower());

// }

//  }

// impl Student {

// fn new() -> Student {

// Student {Name: String::from("Ghufran"), Grade: String::from("A"), Age: 27, Percentage: 74.9}

// }

// fn PercentageShower(&self) -> f32 {

//        self.Percentage

// }

// }  

// fn main() {

//     let studentdetails =  Student::new();

//     println!("{:#?}",studentdetails); 

//     println!("Percentage is {}",studentdetails.PercentageShower());

// }



// #[derive(Debug)]
// enum Message{
// 	Quit,
// 	Move{x:i32,y:u8},
// 	Write(String),
// 	ChangeColor(i32,i32,i32),
// }
// fn main(){
// 	impl Message{
// 		fn call(&self){

//             match self {
//                 Message::Move{x,y} => print!("{}",((*x as u8)*y)),
//                 _ => print!("nothing"),
//             }

// 			// self.move.x*self.move.y
// 		}
// 	}
// 	let m=Message::Move{x:10,y:10};
// 	m.call();
// 	// println!("{:?}",m );
// }

// use std::time::{Duration, Instant};
// use std::thread;

// fn main(){
//     let start = Instant::now();
// let array= [0,1,2,3,4,5,6,7,8,9,10,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,3,3,32,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3,3];

// for i in 0..array.len(){
//     if array[i] == 0{
//         print!("found");
//         break
//     }
// }
// let duration = start.elapsed();
//   println!("Time elapsed in expensive_function() is: {:?}", duration);
// }



// crate text_io;
// fn main() {

    
// }


// use std::io;
// fn main(){
//     let mut my_string = String::new();
//     println!("Enter a number : {}",my_string);
//     io::stdin().read_line(&mut my_string); 
//     my_string = my_string.trim().to_string();

//     let mut sum = 0;
//     for i in 0..my_string.len(){
//         let x : i32 = my_string[i..i+1].parse().unwrap();
//         sum = sum + x;
//     }
//     println!("{}",sum);
// }





// #[derive(Debug,Clone)]
// struct User{
//     username: String,
//     email: String,
//     age : i8,
//     single  : bool,
// }

// impl User {
//     fn user(&mut self) -> String{
//     //    self.username = "awais".to_string();
//     self.username.to_string()
//     }

//     fn user_name(info:User)->String{
//         info.username.to_string()
//     }
// }

// fn main(){

//     let mut user1 = User{
//         username : "jamshaid".to_string(),
//         email : " jamsdf".to_string(),
//         age : 20,
//         single : true,

//     };

    

   

//     println!("{}",user1.user());
//      println!("{}",User::user_name(user1));
//     // print!("{:?}",user2);


// }




// 
// use std::io;
// fn main(){

//     let mut input = String::new();
//     let mut target = String::new();

//     io::stdin().read_line(&mut input);
//     io::stdin().read_line(&mut target);
//     let input = input.trim();
//     let target = target.trim().parse().unwrap();
    
//     let mut anyvec = vec![];

//     for i in input.split_whitespace(){
//         let x : i32 = i.parse().unwrap();
//         anyvec.push(x);
//     }

//     // let x = [1,2,3,4,5];
//     // let target = 6;
//     print!("{:?}",solve(&anyvec, target));
// }
//     fn solve(x:&[i32],target:i32)->(&i32,&i32){
//         let mut anyvec = vec![];

    
// let mut c=0;
//     for i in x.iter(){
//         if c>0{
//             break}
//         for j in x.iter(){

//             if c>0{
//             break
//             }

//             if i+j == target{
//                 let m=i;
//                 let n=j;
//                 anyvec.push(m);
//                 anyvec.push(n);
//                 c+=1;
//                 break
//             }
//         }
//         }
    
//     // print!("{:?}", anyvec);
//     (anyvec[0],anyvec[1])
    
    
    
// }



// use std::io;
// struct Person{
//     name : String,
//     age : String,
//     country : String,
// }

// fn main(){
//     let mut name = String::new();
//     let mut age = String::new();
//     let mut country = String::new();

//     io::stdin().read_line(&mut name);
//     io::stdin().read_line(&mut age);
//     io::stdin().read_line(&mut country);

//     let name = name.trim().to_string();
//     let age = age.trim().to_string();
//     let country = country.trim().to_string();

//     let person1 = Person{
//         name : name,
//         age : age,
//         country : country,
//     };

//     let array = [person1.name,person1.age,person1.country];

//     println!("{:?}",array);


// }









// fn main() {
//     println!("{}",(2 as f32).powf(3 as f32));
// }










// use std::io;
// fn main(){

//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     let input = input.trim();


//     let mut anyvec = vec![];

//     for i in 0..input.len(){
//         let x = &input[i..i+1];
//         anyvec.push(x);

//         print!("{}",x);
//         if input.len() == i+1{
//             break
//         }
//         print!("+");
//     }

//     let mut sum : i32 = 0;
//     for i in anyvec.iter(){
//         let x : i32= i.parse().unwrap();
//         sum = sum +x;
//     }
//     println!("\n{}",sum);

// }










fn main(){

// let name = String::from("hello");

struct String{
    vec : Vec<&u8>
}

impl String{

}

let name = String{
    vec : vec!["h".as_bytes(),"e".as_bytes(),"l".as_bytes(),"l".as_bytes(),"o".as_bytes()],
};

}