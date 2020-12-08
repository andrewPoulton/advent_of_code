use std::collections::HashSet;
use crate::utils::file2vec;

pub fn day8(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    
    let mut ptr: i32 = 0;
    let mut acc: i32 = 0;
    let mut visited: HashSet<i32> = HashSet::new();
    
    let mut part2_ans: Option<i32> = None;
    // let mut 
    loop {
        let instruction = contents[ptr as usize].to_owned();
        let vals: Vec<&str> = instruction.split(' ').collect();
        match vals[0] {
            "acc" => {
                ptr += 1;
                acc += vals[1].parse::<i32>().unwrap();
            },
            "jmp" => {
                let jmp_val= vals[1].parse::<i32>().unwrap();
                match part2_ans {
                    Some(x) => (),
                    None => {
                        if jmp_val < 0 {
                            part2_ans = branch(ptr+1, &contents, &visited, &acc);
                        }
                    }
                }
                ptr+=jmp_val;
            }
            _ => {
                let nop_val= vals[1].parse::<i32>().unwrap();
                match part2_ans {
                    Some(x) => (),
                    None => {
                        if nop_val != 0 {
                            part2_ans = branch(ptr+nop_val, &contents, &visited, &acc);
                        }
                    }
                }
                ptr += 1;
                
            }
        };
        if visited.contains(&ptr){
            println!("acc is {}", acc,);
            break
        } else {
            visited.insert(ptr);
        }
    }
    println!("part 2 ans {}", part2_ans.unwrap_or(-1));
}

fn branch(ptr: i32, contents: &Vec<String>, visited: &HashSet<i32>, acc: &i32)->Option<i32>{
    let mut new_ptr: i32 = ptr;
    let mut new_acc: i32 = *acc;
    // let mut negjmp: i32 = 0;
    // let mut nops: i32 = 0;
    let mut new_visited: HashSet<i32> = HashSet::new();
    new_visited.insert(new_ptr);
    while new_ptr < contents.len() as i32 {
        let instruction = contents[new_ptr as usize].to_owned();
        let vals: Vec<&str> = instruction.split(' ').collect();
        match vals[0] {
            "acc" => {
                new_ptr += 1;
                new_acc += vals[1].parse::<i32>().unwrap();
            },
            "jmp" => {
                let jmp_val= vals[1].parse::<i32>().unwrap();
                new_ptr += jmp_val;
                // negjmp += if jmp_val < 0 {1} else {0};
            }
            _ => {
                new_ptr += 1;
                // let nop_val= vals[1].parse::<i32>().unwrap();
                // nops += if nop_val == 0 {0} else {1};
            }
        };
        if visited.contains(&new_ptr) | new_visited.contains(&new_ptr) | (new_ptr < 0){
            return None
        } else {
            new_visited.insert(new_ptr);
        }
    }
    Some(new_acc)
}