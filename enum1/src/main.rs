
#![allow(unused_variables)]
fn main() {
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

let y= value_in_cents(Coin::Penny);

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

println!("{}",y);

}