
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


/*
use std::fmt::Display;
#[derive(Debug)]
enum Vehicle{
    Mehran,
    Corolla,
}
enum VehicleType{
    Diesal,
    Petrol,
}
fn main() {
    let car = Vehicle::Mehran;
    let car1 = VehicleType::Petrol;
    let x= hp(car,car1);
    print!("{}",x );
}
   
fn hp (Vehicle : Vehicle,VehicleType: VehicleType) -> u32 {

    match Vehicle {
        Vehicle::Corolla => 
         match VehicleType {
             VehicleType::Diesal => 20,
             VehicleType::Petrol => 30,
         },
        Vehicle::Mehran => 
         match VehicleType {
             VehicleType::Diesal => 40,
             VehicleType::Petrol => 50,
         },
    }
}
*/
