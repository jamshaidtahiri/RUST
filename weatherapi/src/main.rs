extern crate reqwest;
use std::collections::HashMap;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

extern crate rustc_serialize;
use rustc_serialize::json::Json;
use std::io::Read;

fn main(){

    let path =  Path::new("api.json");
    let display = path.display();

    print!("{:?},{}",path,display);

    let mut file = match File::create(path){
        Err(why) => panic!("couldn't create {}: {}",display,why.description()),
        Ok(file) => file,
    };

   

    
    
    match reqwest::get("http://api.weatherstack.com/current?access_key=d24ddee8ec749c2a34da5b76645d9d0f&query=Karachi") {
        Ok(mut response)=>{            
            match response.text() {
                Ok(text)=> match file.write_all(text.as_bytes()){
                                Err(why) => {panic!("couldn't write to {}: {}", display,why.description())},
                                Ok(_) => println!("successfully wrote to {}", display)},
                Err(_) => println!("Error in Text"),
            }                    
        },
        Err(_)=> println!("Server Down")
    }    




    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    // match file.read_to_string(&mut s) {
    //     Err(why) => panic!("couldn't read {}: {}", display,
    //                                                why.description()),
    //     Ok(_) => print!("{} contains:\n{}", display, s),
    // }
    file.read_to_string(&mut s).unwrap() ;


let json = Json::from_str(&s).unwrap();
    println!("{} degree", json.find_path(&["current","temperature"]).unwrap());
}
 
