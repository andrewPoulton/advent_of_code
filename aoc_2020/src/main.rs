use std::env;
use aoc_2020::*;

use std::time::{Instant};


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
        "15" => day15(&fname),
        "16" => day16(&fname),
        "17" => day17(&fname),
        "18" => day18(&fname),
        "19" => day19(&fname),
        "test.txt" => day19(&"test.txt".to_string()),
        _ => {
            println!("Day {} is not a potential_fields yet, or ever", args[1]);
        }
    };
    let duration = start.elapsed();
    println!("overall it took {:?}", duration);

    
}

use std::collections::HashMap;
pub fn day19(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();
    let mut rules  = HashMap::new();
    let mut messages = Vec::new();
    for line in &contents{
        if line.contains(':') {
            let mut key = "";
            // let mut key = String::from("");
            for (i, s) in line.split(':').enumerate(){
                if i == 0 {
                    key = s;
                } else {
                    rules.insert(key.to_owned(), s.trim().to_owned());
                }
            }
        } else if line.len()>0{
            messages.push(line.to_owned());
        };
    };

    println!("{:?}", rules);
    println!("{:?}", messages);

}
