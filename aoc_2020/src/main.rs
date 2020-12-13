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
        "test.txt" => day13(&"test.txt".to_string()),
        _ => {
            println!("Day {} is not a thing yet, or ever", args[1]);
        }
    };
    let duration = start.elapsed();
    println!("overall it took {:?}", duration);
}


fn day13(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    
    let ts: &i32 = &(contents)[0].parse::<i32>().unwrap();
    let mut part_1_time = std::i32::MAX;
    let mut part_1_idx = std::i32::MAX;
    let buses: Vec<(usize, i128)> = contents[1].split(',').map(| x|{
        // in part 1 we find the earliest departure time after our timestamp ts by taking the timestamp modulo bus idx
        // we also collect the buses into (idx, i128) tuples for part 2
        match x {
            "x" => None,
            _ => {
                let elt = Some(x.parse::<i32>().unwrap());
                let time = elt.unwrap() - *ts%elt.unwrap();
                if time < part_1_time {
                    part_1_time = time;
                    part_1_idx = elt.unwrap();
                }
                elt
            }
        }
    }).enumerate()
    .filter(|(idx, elt)|{
        match elt {
            Some(c) => true,
            None => false
        }
    }).map(|(idx, elt)| (idx, elt.unwrap() as i128)).collect();

    
    println!("part 1 ans: {}", part_1_time*part_1_idx );


    // given bus tuples (i_0, n_0),..., (i_m, n_m) we want to find x s.t.
    // (x+i_j) % n_j == 0 for all j (*)
    // for each j, x = -i_j is such a number, and so is -i_j*(n_k*n_k^{-1}_j) etc, 
    // where n_k^{-1}_j is the inverse of n_k in Z/n_jZ (so n_k*n_k^{-1} % n_j == 1 ) for all k != j
    // so if x = -Sum_j i_j*(Prod_{k!=j}n_k*n_k^{-1}_j) (**), 
    // then x % n_j == -i_j for all j, as desired
    // we need the least positive X satisfying (*), and since all bus ids are prime, x is a residue mod Prod_j n_j = bus_prod
    // so the least positive X is bus_prod - (x% bus_prod) since we actually compute -x from (**), not x
    // we compute (**) in a double loop (so we are O(buses^2)), 
    // I don't think the inner product can be less than O(buses) as we need to find inverses as well as the products.
    let mut offset: i128 = 0;
    let mut bus_prod = 1 as i128;
    for (_, bus) in &buses {
        bus_prod *= *bus;
    }
    for (idx, bus) in &buses{
        let mut inverses = *idx as i128;
        for (i, other) in &buses {
            
            // avoid overflows
            inverses %= bus_prod as i128;
            
            inverses *= if i != idx {
                let inv = get_mult_inverse(*other, *bus);
                *other * inv
            } else {1} 
        }
        offset += inverses;
    }
    println!("part 2 ans: {}",bus_prod- (offset% bus_prod));
}


fn get_mult_inverse(x:i128, p:i128)->i128{
    // find the inverse of x in Z/pZ
    let mut tmp_x = x;
    let mut inv = 0;
    while tmp_x != 1 {
        inv = tmp_x;
        tmp_x = (tmp_x*x) % p;
    }
    inv
}


