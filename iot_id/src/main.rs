extern crate menu_generator;
use menu_generator::read_input as io; 
use menu_generator::student_registration as sg;
fn main() {
    println!("ENter your name:");
    let name:String=io::str_read();
    println!("ENter your course:");
    let course:String=io::str_read();
    println!("ENter your city:");
    let city:String=io::str_read();
    println!("ENter your distance learning:");
    let distance_learning:String=io::str_read();
   let student1=sg::Student::register(name,course,city,distance_learning);
    student1.display_card();

}
