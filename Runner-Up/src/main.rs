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
