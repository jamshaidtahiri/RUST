use std::io;
fn main() {
    let mut anyvec = vec![];
    loop{
    let mut anyvecelement = String::new();
    io::stdin().read_line(&mut anyvecelement );
    if anyvecelement == "\n"{
        break;
    }
    let anyvecelement : i32 = anyvecelement.trim().parse().unwrap();
    
    anyvec.push(anyvecelement);
    }
    anyvec.sort();
    print!("{:?}",anyvec);
    
    let mut x = anyvec[0];
    for i in anyvec.iter(){
    if i > &x{
        let second_max = i;
        print!("{}",second_max );
        break;
    }
    }
    





}
