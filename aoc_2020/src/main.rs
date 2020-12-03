use std::env;
use aoc_2020::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    day3(&args[1]);
}

