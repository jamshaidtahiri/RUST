fn main(){

let pi = String::from("314159265358979323846264338327950288419716");
let fav_arr = ["141","6535","592","3238462643","327950288419716"];

let mut ivec=vec![];
let mut jvec = vec![];

let str_len = pi.len();
for i in 0..str_len{
    for j in i..str_len+1{
        let m=&pi[i..j] ;
        for &k in fav_arr.iter(){
            if m == k{
                ivec.push(i);
                jvec.push(j);
               }
        }
    }
}

println!("{:?}\n{:?}",ivec,jvec );
let k = 0;
let mut anyvec = vec![];
for i in 0..ivec.len(){
    // let m =ivec[i];
    // let n = jvec[i];
    if i == 0{
    let k = 0;

    if ivec[i]-0 > 0{
        anyvec.push(&pi[0..ivec[i]]);
    }

    }
    else {
       let k = jvec[i-1];
       if ivec[i] as i32-k as i32 > 0{
        anyvec.push(&pi[jvec[i-1]..ivec[i]]);
    }
    }

    

    anyvec.push(&pi[ivec[i]..jvec[i]]);
    anyvec.push(" ");
}
    


println!("{:?}",anyvec );
println!("{:?}",anyvec.join(""));

}

