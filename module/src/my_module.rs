


#[derive(Debug)]
pub struct Login {
    name: String,
    password: String, 
}

impl Login {
    pub fn login1(name : String,password: String)->Login{
        Login{
            name,
            password,
        }

    }
}



// pub mod inner_module{
//     pub fn print_hello () -> String {
//         String::from("Hello from inner module under my module")
//     }
// }


