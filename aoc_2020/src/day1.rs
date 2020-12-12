use std::collections::HashMap;
use std::time::Instant;
use crate::utils::file2vec;

pub fn day1(filename: &String)->(){
    
    let mut contents:Vec<i32> = file2vec::<i32>(filename)
    .iter().map(|x|x.to_owned().unwrap()).collect();
    
    let mut map: HashMap<i32,usize> = HashMap::new();
    for i in 0..contents.len(){
        map.insert(2020-contents[i], i);
    }
    let mut part1_solution: Option<i32> = None;
    for i in 0..contents.len(){
        let ans = match map.get(&contents[i]) {
            Some(ans) => {
                (contents[i], contents[*ans])
            },
            None => (-1,-1)
        };
        if ans.0>=0{
            println!("solution p1 is {}, {}", ans.0, ans.1);
            part1_solution = Some(ans.0*ans.1);
            break
        }
    }
    let start = Instant::now();
    let mut part2_solution: Option<i32> = None;
    contents.sort();
    for i in 0..contents.len()-2 {
        let mut j = i+1;
        let mut k = contents.len()-1;
        while j<k {
            let ans = contents[i] + contents[j] + contents[k];
            part2_solution = if ans == 2020 {
                    println!("solution p2 is {}, {}, {}", contents[i], contents[j], contents[k]);
                    Some(contents[i]*contents[j]*contents[k])
                }else if ans > 2020 {
                    k-=1;
                    None
                } else{
                    j+=1;
                    None
                };
            match part2_solution {
                None => (),
                _ => break
            };
        }
        if j<k{
            break
        }
    }
    println!("3sum took {:?}", start.elapsed());
    

    println!("Day 1 Part 1: {}", part1_solution.unwrap_or(-1));
    println!("Day 1 Part 2: {}", part2_solution.unwrap_or(-1));    
}