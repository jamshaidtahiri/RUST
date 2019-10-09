//  pub mod math {
    
// use std::io;
// pub fn factorial(){
// 	let mut number=String::new();
// 	println!("Please enter the number");
// 	io::stdin().read_line(& mut number).expect("incorrect entry");
// 	let  mut number:u64=number.trim().parse().unwrap();
//     println!("{}",fact(number));
// }
// fn fact(input:u64)->u64{
// if input == 0{
//     1
// }
// else {
//     input*fact(input-1)
// }
// }
// }

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {

	
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
	
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
