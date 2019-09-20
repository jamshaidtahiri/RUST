// use std::io;
// fn main() {
    
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     // let input = input.trim().parse().unwrap();
    

//     for i in input.split_whitespace(){
//     println!("{}",i );
//     }

// }

// use std::io;
// fn main(){
//     let mut int = String::new();
//     io::stdin().read_line(&mut int);
//     let int : i32 = int.trim().parse().unwrap();
    
//     let mut anyvec = vec![]; 
//     for i in 0..int{
//         let  mut input = String::new();
//     io::stdin().read_line(&mut input);
//     let input : i32 = input.trim().parse().unwrap();
//     anyvec.push(input);
//     }
//     println!("{:?}",anyvec);
// }

use std::io;
fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // let mut input = input.trim();
let mut anyvec = vec![];
let mut vector : Vec<i32> = vec![];

    for i in 0..input.len(){
        let num = &input[i..i+1];
        if num == " " || num == "\n"{
            let n = anyvec.join("");
            let n:i32 = n.parse().unwrap();
            vector.push(n);
            for _ in 0..anyvec.len(){
                anyvec.pop();
            }            

        }else {
            anyvec.push(num);
        }
                
    }
println!("{:?}",vector);
}