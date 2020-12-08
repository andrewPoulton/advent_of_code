use std::env;
use aoc_2020::*;

// use std::collections::{HashMap, HashSet};

fn main() {
    let args: Vec<String> = env::args().collect();

    day7(&args[1]);
}
