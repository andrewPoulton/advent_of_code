use std::env;
use aoc_2020::*;

use std::time::{Instant};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();
    let fname = format!("src/day{}.txt", args[1]);
    match &args[1][..]{
        "1" => day1(&fname),
        "2" => day2(&fname),
        "3" => day3(&fname),
        "4" => day4(&fname),
        "5" => day5(&fname),
        "6" => day6(&fname),
        "7" => day7(&fname),
        "8" => day8(&fname),
        "9" => day9(&fname),
        "10" => day10(&fname),
        "11" => day11(&fname),
        "12" => day12(&fname),
        "13" => day13(&fname),
        "14" => day14(&fname),
        "test.txt" => day14(&"test.txt".to_string()),
        _ => {
            println!("Day {} is not a thing yet, or ever", args[1]);
        }
    };
    let duration = start.elapsed();
    println!("overall it took {:?}", duration);
}
 
fn day14(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();


    let mut addresses_pt1= HashMap::new();
    let mut addresses_pt2= HashMap::new();
    let mut mask : String = String::from("");
    let mut key: String = String::from("");
    for line in &contents{
        let thing: Vec<&str> = line.split(" = ").collect();
        match thing[0] {
            "mask" => {mask = thing[1].to_owned()},
            _ => {
                key = thing[0].to_owned();
                let num = thing[1].parse::<i64>().unwrap();
                let value = apply_mask(&mask, num);
                addresses_pt1.insert(key, value);
                let pt2_addresses = get_addresses(&thing[0], &mask);
                for address in &pt2_addresses {
                    addresses_pt2.insert(*address, num);
                }
            }
        }
    }
    let part1_ans = addresses_pt1.iter().fold(0, |mut acc, (k,v)| {
        acc += *v;
        acc
    });
    let part2_ans = addresses_pt2.iter().fold(0, |mut acc, (k,v)| {
        acc += *v;
        acc
    });
    println!("part 1 ans: {}", part1_ans);
    println!("part 2 ans: {}", part2_ans);
}


fn to_binary_str(x:i64)-> String{
    format!("{:036b}", x)
}

fn from_binary_str(s: &str) -> i64 {
    i64::from_str_radix(s, 2).unwrap_or(-1)
}

fn apply_mask(mask: &str, x: i64)->i64{
    let char_array: Vec<char> = to_binary_str(x).chars().collect();
    let mut ans: String = String::from("");
    for (idx, bit) in mask.char_indices(){
        match bit {
            '0' | '1' => {ans.push(bit);},
            _ => {ans.push(char_array[idx]);}
        }
    }
    from_binary_str(&ans[..])
}

fn extract_memory_address(mem: &str) -> i64 {
    mem[4..mem.len()-1].parse::<i64>().unwrap_or(-1)
}

fn get_addresses(mem: &str, mask: &str) -> Vec<i64>{
    let initial_address = extract_memory_address(mem);
    let char_array: Vec<char> = to_binary_str(initial_address).chars().collect();
    let mut ans: Vec<String> = vec![String::from("")];
    for (idx, bit) in mask.char_indices(){
        match bit {
            '0' => {
                for s in &mut ans{
                    s.push(char_array[idx]);
                }
            },
            '1' => {
                for s in &mut ans{
                    s.push(bit);
                }
            },
            'X' => {
                // println!("{:?}", ans);
                let l = ans.len();
                for i in 0..l {
                    let mut branch = ans[i].clone();
                    ans[i].push('0');
                    branch.push('1');
                    ans.push(branch);
                }
            }, _ => ()
        }
    }
    ans.iter().map(|x|from_binary_str(&x[..])).collect()
}