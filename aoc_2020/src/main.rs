use std::env;
use aoc_2020::*;

use std::time::{Instant};

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();
    day11(&args[1]);
    let duration = start.elapsed();
    println!("overall it took {:?}", duration);
}