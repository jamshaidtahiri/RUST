extern crate my_lib;
// extern crate text_io;
use std::io;
fn main() {
//    let mut name =  String::new();
//    let mut father_name = String::new();
//    let mut age = String::new();
//    let mut roll_no = String::new();

//    io::stdin().read_line(&mut name);
//     let name = name.trim().to_string();
//    io::stdin().read_line(&mut father_name);
//     let father_name= father_name.trim().to_string();
//    io::stdin().read_line(&mut age);
//     let age : u64 = age.trim().parse().unwrap();
//    io::stdin().read_line(&mut roll_no);
//     let roll_no : u64 = roll_no.trim().parse().unwrap();


//    my_lib::id_generate(name,father_name,age,roll_no);

my_lib::generate_id();

}
