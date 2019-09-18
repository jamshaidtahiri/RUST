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
     
//     for i in new_vector.iter(){
        
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

//