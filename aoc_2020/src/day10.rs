use std::time::{Duration, Instant};
use crate::utils::file2vec;

pub fn day10(filename: &String){
    let contents = file2vec::<i32>(filename);
    let mut contents:Vec<i32> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    contents.sort();
    let start = Instant::now();
    
    let mut ones = 0;
    let mut threes = 0;
    let mut prev = 0;
    let mut diffs = Vec::new();
    diffs.push(3); //i.e. pretend there's -3 jolts before the charging outlet
    for x in &contents{
        let diff = x-prev;
        match diff {
            3 => threes += 1,
            1 => ones += 1,
            _ => ()
        };
        diffs.push(diff);
        prev = *x;
    }
    let duration = start.elapsed();
    println!("part 1 ans {}", ones*(threes+1));
    println!("took {:?}", duration);
    let start = Instant::now();
    diffs.push(3); // diff of 3 for device

    
    let mut idx:usize = 0;
    let mut ans:i64 = 1;
    while idx < diffs.len()-1{
        let mut j:usize = idx+1;

        // increment along diffs while we see diffs of 1
        while let 1 = diffs[j] {
            j+=1;
        }
        
        match j-idx+1{
            6 => ans *=7, //3,1,1,1,1,3 => 7 possible reformulations
            5 => ans *=4, // 3,1,1,1,3 => 4 possible reformulations
            4 => ans *= 2, // 3,1,1,3 => 2 possible reformulations
            _ => () // 3,1,3 or 3,3 => no possible reformulations
        };
        idx = j;
    }
    let duration = start.elapsed();
    println!("part 2 ans: {}", ans);
    println!("took {:?}", duration);
}

// different options for diff slices and what they correspond to 

//3,1,1,1,1,3 -- 3,4,5,6,7,10
//3,2,1,1,3 -- 3,5,6,7,10
//3,1,2,1,3 -- 3,4,6,7,10
// 3,3,1,3 -- 3,6,7,10
//3,1,3,3 -- 3,4,7,10
// 3,2,2,3 -- 3,5,7,10
// 3,1,1,2,3 -- 3,4,5,7,10

// 3,1,1,1,3
// 3,2,1,3
// 3,1,2,3
// 3,3,3

// 3,1,1,3
// 3,2,3

// 3,1,3

