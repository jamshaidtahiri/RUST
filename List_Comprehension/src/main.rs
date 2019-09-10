use std::io;

fn main() {
    let mut x = String::new();
    io::stdin().read_line(&mut x);   
    let x : i32 = x.trim().parse().unwrap();
    let mut y = String::new();
    io::stdin().read_line(&mut y);   
    let y : i32 = y.trim().parse().unwrap();
    let mut z = String::new();
    io::stdin().read_line(&mut z);   
    let z : i32 = z.trim().parse().unwrap();
    let mut n = String::new();
    io::stdin().read_line(&mut n);   
    let n : i32 = n.trim().parse().unwrap();

    let mut list = vec![];

    for i in 0..x+1{
        for j in 0..y+1{
            for k in 0..z+1{
                if i+j+k != n{
                    list.push([i,j,k])
                }
            }
        }
    }



println!("{:?}",list);
}
