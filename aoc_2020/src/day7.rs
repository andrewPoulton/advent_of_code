use std::collections::{HashMap, HashSet};
use crate::utils::file2vec;


pub fn day7(filename:&String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();

    let mut contained_in_map: HashMap<String,Vec<String>> = HashMap::new();
    let mut contains_map: HashMap<String,Vec<(String, i32)>> = HashMap::new();
    for line in contents.iter(){
        
        let curr = parse_line(line);

        match curr.len() {
            1 => (),
            _ => {
                for (key, quantity) in curr[1..].iter(){
                    let containers = contained_in_map.entry(key.to_string()).or_insert(Vec::new());
                    containers.push(curr[0].0.to_owned());

                    match contains_map.get_mut(&curr[0].0) {
                        Some(container) => {
                            container.push((key.to_owned(), *quantity));
                        },
                        None => {
                            contains_map.insert(curr[0].0.to_owned(), vec![(key.to_owned(), *quantity)]);
                        }
                    }

                }
            }
        }
    };
   
    let mut stack = vec!["shiny gold".to_string()];
    let mut seen = HashSet::new();
    let mut count = 0;
    while stack.len()>0 {
        match stack.pop() {
            Some(bag) => {
                match contained_in_map.get_mut(&bag) {
                    Some(bags) => {
                        for new_bag in bags{
                            if !seen.contains(new_bag){
                                seen.insert(new_bag.to_owned());
                                count += 1;
                            }
                            
                            stack.push(new_bag.as_str().to_owned());
                        }
                    },
                    None => ()
                };
            },
            None => ()
        }
    };
    println!("{} bags can hold shiny gold bags", count);

    let mut stack = vec![("shiny gold".to_string(),1 as i32, 0 as i32)];
    let mut full_count = 0;
    while  stack.len() > 0 {
        match stack.pop(){
            Some(bag) =>{ //parent, 
                
                let mut sub_count = 0;
                match contains_map.get(&bag.0) {
                    Some(bags) =>{
                        for b in bags{
                            sub_count += b.1*bag.1;
                            stack.push((b.0.to_owned(), b.1*bag.1, bag.1));
                        }
                    },
                    None => ()
                }
                full_count += sub_count;
                
            },
            None => ()
        }
    }
    println!("full count {}", full_count);
}

fn parse_line(line: &String)-> Vec<(String, i32)>{
    // hash map contains <bag: Vec<bag_contained_in, quantity>>
    let mut map = Vec::new();
    line.split(" contain ").enumerate().fold(map, |mut acc, (idx,bag)|{
        match idx {
            0 => { //the containing bag, store as new value to ad
                let new_val = bag.trim().split("bag").next().unwrap().trim().to_owned();
                acc.push((new_val, 0));
                acc
            },
            _ => { // bags contained by new_val bag
                match bag.trim() {
                    "no other bags." => {
                        acc
                    },
                    bag_val => {
                        for (sub_bag, quantity) in bag_val.split(",").map(|bags| {
                            let bag_type = bags.trim()[1..].trim().split("bag").next();
                            (bag_type.unwrap().trim().to_owned(), bags.trim()[..1].parse::<i32>().unwrap_or(-1)  ) 
                        }){
                            acc.push((sub_bag.to_owned(), quantity));
                        }   
                        acc
                    }
                }
            }
        }
    })
}