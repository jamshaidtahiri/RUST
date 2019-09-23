// mod my_module;
// fn main() {
//     println!("{}",my_module::inner_module::print_hello());
// }



mod my_module;
use my_module::Login;
fn main() {
    let name = String::from("jamshaid");
    let password = String::from("password");
    let user1 = Login::login1(name,password);
    print!("{:?}",user1);
}
