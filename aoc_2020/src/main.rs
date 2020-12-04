use std::env;
use aoc_2020::*;


fn main() {
    let args: Vec<String> = env::args().collect();

    day4(&args[1]);
}