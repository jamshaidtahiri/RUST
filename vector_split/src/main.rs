use std::io;
fn main() {
    
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    // let input = input.trim().parse().unwrap();
    

    for i in input.split_whitespace(){
    println!("{}",i );
    }

}
