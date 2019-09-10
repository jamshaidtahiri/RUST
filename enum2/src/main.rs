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
