use std::io;

fn main() {
    // io::stdin().read_line(& mut list)
    let mut list = vec![];
    let mut x = String::new();
    io::stdin().read_line(& mut x);
    let x=x.trim();
    for i in 0..x.len(){
    let y = &x[i..i+1]; 
    let y : i32= y.parse().unwrap();
    list.push(y) 
    }
    // println!("{:?}",vector );
    // let mut list = vec![2,3,7,5,3,7];
    let max = longest(&list);
    // let mut repetition= 0;
    // for i in 0..list.len(){
    //     if max == list[i]{
    //         repetition+=1;
    //     }
    // }

    //let mut y = 1;
    while max == longest(&list){
     // let x = repetition-1;
    for i  in 0..(list.len()){
        if max == list[i]{
            list.remove(i);
            break;
        }
        //y = y+1
    }
}
    let max = longest(&list);
    print!("{}",max);

   
}
fn longest(a : &Vec<i32>) -> i32{
    let mut longest =a[0];
    for &i in a.iter(){

        if i>longest{
            longest = i;
        }
    }
    longest
}


/* 
use std::io;
fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1);
    let mut input1 : i32 = input1.trim().parse().unwrap();
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2);
    let mut input2 : i32 = input2.trim().parse().unwrap();
    
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3);
    let mut input3 : i32 = input3.trim().parse().unwrap();
    
    let mut input4 = String::new();
    io::stdin().read_line(&mut input4);
    let mut input4 : i32 = input4.trim().parse().unwrap();
    
    let mut input5 = String::new();
    io::stdin().read_line(&mut input5);
    let mut input5 : i32 = input5.trim().parse().unwrap();

    let mut array = [input1,input2,input3,input4,input5];

    array.sort();
    let x = array.len();

    println!("{:?}",array[x-1]);

 
}
*/
