use crate::utils::file2vec;
use std::collections::HashMap;
use std::time::Instant;

pub fn day15(filename: &String){
    let contents = file2vec::<String>(filename);
    let start_list: Vec<i32> = contents[0].to_owned().unwrap().split(',').map(|x|x.parse::<i32>().unwrap()).collect();

    let mut map = HashMap::new();
    // let mut counts = HashMap::new();

    let mut turns: i32 = 0;
    let mut prev = 0;
    let mut next = 0;
    let mut seen = Vec::new();
    for num in start_list.iter(){
        turns += 1;
        let (prev_turn, count) = map.entry(*num).or_insert((turns, 0));
        
        prev = *num;
        *count += 1;
        seen.push(*num);
    }
    let (p1, p2) = faster_more_memory(&start_list, prev as usize, 30_000_000, 2020);
    println!("part 1 ans: {}\npart 2 ans: {}", p1, p2);
    
    //Saving below for posterity - my initial slower solution. Took about 2s when built for release (30s otherwise)
    // Only major difference is that we index into a hashmap rather than an array - those constants matter!

    /*while turns < 30_000_000{
        turns += 1;
        match map.get(&prev){
            Some((prev_turn, count)) => {
                if (*prev_turn == turns-1) & (*count == 1) {
                    // last time we saw prev was the first time we saw it
                    next = 0;
                } else if *prev_turn == turns-1{
                    next = turns - *prev_turn;
                    let (prev_turn, count) = map.entry(prev).or_insert((turns, 0));
                    *count +=1;
                    *prev_turn = turns-1;
                } else {
                    next = turns - *prev_turn -1 ;
                    let (prev_turn, count) = map.entry(prev).or_insert((turns, 0));
                    *count +=1;
                    *prev_turn = turns-1;
                }
            },
            None => {
                next = 0;
                let (prev_turn, count) = map.entry(prev).or_insert((turns, 0));
                    *count +=1;
                    *prev_turn = turns-1;
                }
            }
            
            if turns == 2020{
                println!("part 1 ans: {}", next);
            }
            
            prev = next;
    }

    println!("part 2 ans: {}", prev);
    let start = Instant::now();
    println!("{}, took {:?}", p1, start.elapsed());*/
    
}

fn faster_more_memory(nums: &Vec<i32>, last: usize, n_turns: usize, p1_turns: usize)-> (usize, usize) {
    let mut buffer = vec![0;n_turns];
    let mut turns:usize = 1;
    let mut p1_ans = 0;
    for num in nums[..nums.len()-1].iter(){
        buffer[*num as usize] = turns;
        turns += 1;
    }
    let mut prev = last;
    let mut cur: usize = 0;
    while turns < n_turns {
        cur = turns - buffer[prev];
        if cur == turns {
            cur = 0;
        }
        buffer[prev] = turns;
        prev = cur;
        turns += 1;
        if turns == 2020{
            p1_ans = cur;
        }
    }
    (cur, p1_ans)
}