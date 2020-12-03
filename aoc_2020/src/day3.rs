use crate::utils::file2vec;

pub fn day3(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    println!("rows: {}, columns: {}", contents.len(), contents[0].len());
    //horizontal counters
    let mut v: usize = 1; // r1d1 
    let mut w: usize = 3; // r3d1 
    let mut x: usize = 5; // r5d1
    let mut y: usize = 7; // r7d1
    let mut z: usize = 0; // r1d2

    //vertical counter
    let mut i: i16 = 0;
    // for i in 0..contents.len()
    let count: Vec<i32> = contents[1..].iter().fold(
        vec![0,0,0,0,0], |mut acc, row| {
            i+=1;
            
            acc[0] += if row.chars().nth(v) == Some('#') {1} else {0};
            acc[1] += if row.chars().nth(w) == Some('#') {1} else {0};
            acc[2] += if row.chars().nth(x) == Some('#') {1} else {0};
            acc[3] += if row.chars().nth(y) == Some('#') {1} else {0};

            if i%2 == 0{
                z = (z+1)%row.len();
                acc[4] += if row.chars().nth(z) == Some('#') {1} else {0};
            }
            
            v = (v+1) % row.len();
            w = (w+3) % row.len();
            x = (x+5) % row.len();
            y = (y+7) % row.len();

            acc
        }
    );
    
    println!("part 1 answer: {}, part 2 answer: {}",count[1],  count.iter().fold(1 as i64, |mut acc, elt| {acc *= *elt as i64; acc}));
}
