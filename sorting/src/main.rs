fn main() {
    
    let somevec = vec![2,1,4,3,6,5];
    let mut emptyvec :  Vec<i32> = vec![];
    let mut onevec = somevec[0];
    for i in 0..somevec.len(){
        
            if onevec < somevec[i]{
                emptyvec.push(somevec[i]);
                let onevec = somevec[i];

            }


        
    }
print!("{:?}",emptyvec );




}
