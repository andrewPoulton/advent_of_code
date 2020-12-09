use std::collections::{VecDeque, HashMap};
use std::time::{Duration, Instant};

use crate::utils::file2vec;

pub fn day9(filename:&String){
    let contents = file2vec::<i64>(filename);
    let contents:Vec<i64> = contents.iter().map(|x| x.to_owned().unwrap()).collect();

    let mut q: VecDeque<i64> = VecDeque::new();
    let start = Instant::now();
    let mut duration: Duration;
    let mut invalid_number:i64 = 0;
    for num in &contents{
        match q.len() {
            25 => {
                let has_sum = two_sum(&q, *num);
                if !has_sum {
                    duration = start.elapsed();
                    println!("part 1 ans {}", *num);
                    println!("time taken: {:?}", duration);
                    invalid_number = *num;
                    break
                } else {
                    q.pop_front();
                    q.push_back(*num);
                }
            },
            _ => {
                q.push_back(*num);
            }
        }
    };
    let start = Instant::now();
    let ans_slice = encryption_weakness(&contents, invalid_number);
    let mut min: i64 = std::i64::MAX;
    let mut max: i64 = std::i64::MIN;
    for x in ans_slice {
        min = min.min(*x);
        max = max.max(*x);
    }
    let duration = start.elapsed();
    println!("Part 2 ans is {}", min+max);
    println!("time taken: {:?}", duration);
}

fn two_sum(q: &VecDeque<i64>, sum:i64)->bool{
    let mut complements: HashMap<i64, usize> = HashMap::new(); // find n+m in q = sum.  complements[sum-m]
    for (idx,num) in q.iter().enumerate(){
        if complements.contains_key(num){
            return true
        } else {
            complements.insert(sum-*num, idx);
        }
    }
    false
}

fn encryption_weakness(nums: &Vec<i64>, target: i64)->&[i64]{
    let mut prefix_sums: Vec<i64> = Vec::new();
    //prefix_sums[i] = nums[..i].sum() -> num[0] + ... + nums[i-1]
    // => prefix_sums[j] - prefix_sums[i] = nums[i] + ... + nums[j-1] when j>i;
    //all nums are positive, so prefix_sums is sorted -> binary search the bugger!
    let mut curr_sum: i64 = 0;
    prefix_sums.push(curr_sum);
    for num in nums{
        curr_sum += *num;
        prefix_sums.push(curr_sum);
    };
    

    let mut i: usize = 0;
    let mut idx: Option<usize> = binary_search_slice(&prefix_sums[i..], target);
    while let None = idx{
        i += 1;
        idx = binary_search_slice(&prefix_sums[i..], target);
    };
    let j = idx.unwrap();
    let answer_slice = &nums[i..i+j];
    answer_slice
}

fn binary_search_slice(slice: &[i64], target: i64) -> Option<usize> {
    let mut left:usize = 0;
    let mut right:usize = slice.len()-1;
    while left <= right {
        let mid = (left + right)/2;
        let sub_sum = slice[mid] - slice[0];
        if sub_sum == target {
            return Some(mid);
        } else if sub_sum < target {
            left = mid +1;
        } else {
            right = mid -1;
        }
    };
    None
}