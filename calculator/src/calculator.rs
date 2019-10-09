use std::io;
use std::io::Write;
pub fn calculator(){
loop{
let stdin = io::stdin();   
print!(">>");
 io::stdout().flush().ok();
 drop(stdin);

let mut string1 = String::new();
let mut string2 = String::new();
let mut character = String::new();



let mut input = String::new();
io::stdin().read_line(&mut input).expect("invalid argument");
input = input.trim().to_string();

let mut sww = remove_whitespace(&mut input);


split_string(&mut sww, &mut string1, &mut string2, &mut character);

// print!("{},{}",string1,string2 );
let result : f64 = 
match character.as_str() {
    "+" => add(string1,string2),
    "-" => subtract(string1,string2),
    "*" => multiply(string1,string2),
    "/" => divide(string1,string2),
    _ => 0.0,

};


println!("{}", result);









}
}


fn remove_whitespace(s: &mut String) ->String {
    s.retain(|c| !c.is_whitespace());
    let y = s;
    y.to_string()
}


fn split_string(data : &mut String,string1:&mut String,string2:&mut String,character:&mut String){
for i in 0..data.len(){
    let k =&data[i..i+1];
    
    if  k == "+" || k =="-" || k =="*" || k =="/" 
    {
        character.push_str(&data[i..i+1]);
        let mut x = i;
        loop{
            if (x+1)==data.len(){
                break
            }
            
            string2.push_str(&data[x+1..x+2]);
            x=x+1;
                 
            
        }
        break
       
    }
    else {
        
         string1.push_str(&data[i..i+1]);
    }
}
}



fn add(string1:String,string2:String)->f64{

    let num1 : f64= string1.parse().unwrap();
    let num2 : f64= string2.parse().unwrap();
 
 num1+num2
    

}
fn subtract(string1:String,string2:String)->f64{

    let num1 : f64= string1.parse().unwrap();
    let num2 : f64= string2.parse().unwrap();
 
 num1-num2
    

}
fn multiply(string1:String,string2:String)->f64{

    let num1 : f64= string1.parse().unwrap();
    let num2 : f64= string2.parse().unwrap();
 
 num1*num2
    

}
fn divide(string1:String,string2:String)->f64{

    let num1 : f64= string1.parse().unwrap();
    let num2 : f64= string2.parse().unwrap();
 
 num1/num2
    

}