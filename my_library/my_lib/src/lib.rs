pub struct Student { 
    name:String,
    father_name:String,
    age:u64,
    roll_no:u64,
}

impl Student{
    pub fn student_id(&self ){
        println!("
                     
                STUDENT ID CARD                
                                          
#################################################            
                                           
        Name : {}                                  
                                         
        Father Naem : {}                  
                                         
        Age : {}                          
                                         
        Roll No : {}                      
                                         
#################################################         
                                         
                     
",self.name,self.father_name,self.age,self.roll_no);

    }
}


// pub fn id_generate(name: String,father_name:String,age:u64,roll_no:u64){
// println!("
                     
//           STUDENT ID CARD                
                                          
//      ############################           
                                           
//      Name : {}                                  
                                         
//      Father Naem : {}                  
                                         
//      Age : {}                          
                                         
//      Roll No : {}                      
                                         
//      ############################        
                                         
                     
// ",name,father_name,age,roll_no);
pub fn generate_id(){

use std::io;
use std::io::prelude::*;
println!("Please enter details");
print!("Name : ");
    io::stdout().flush().ok().expect("Could not flush stdout");

let mut name =  String::new();

   let mut father_name = String::new();
   
   let mut age = String::new();
   
   let mut roll_no = String::new();

   io::stdin().read_line(&mut name).expect("reinput");
    let name = name.trim().to_string();

    print!("Father Name :");
io::stdout().flush().ok().expect("Could not flush stdout");
   io::stdin().read_line(&mut father_name).expect("reinput");
    let father_name= father_name.trim().to_string();

    print!("Age :");
io::stdout().flush().ok().expect("Could not flush stdout");
   io::stdin().read_line(&mut age).expect("reinput");
    let age : u64 = age.trim().parse().unwrap();

    print!("Roll No :");
io::stdout().flush().ok().expect("Could not flush stdout");
   io::stdin().read_line(&mut roll_no).expect("reinput");
    let roll_no : u64 = roll_no.trim().parse().unwrap();

let student = 
    Student{
        name,
        father_name,
        age,
        roll_no,

    };

    student.student_id();










    }


