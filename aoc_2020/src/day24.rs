use std::collections::HashSet;
use crate::utils::file2vec;

pub fn day24(filename:&String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter()
    .filter_map(|x| {
        match x {
            Ok(line) => {
                if !line.is_empty(){
                    Some(line.to_owned())
                } else {None}
            },
            _ => None
        }
    }
    ).collect();
    let mut seen = HashSet::new();
    
    for line in &contents{
        let tile = find_tile(line);
        
        if seen.contains(&tile){
            
            
            seen.remove(&tile);
        } else {
            seen.insert(tile);
        }
    }
    println!("Part 1 ans: {}", seen.len());

    let directions: Vec<(i32,i32)> = vec!["se", "sw", "nw", "ne", "e", "w"].iter().map(|x|find_tile(*x)).collect();
    let mut day = 0;
    while day < 100{
        seen = tick_day(&seen, &directions);
        day+=1;
        
    }
    println!("part 2 ans: {}", seen.len());

}

fn find_tile(line: &str)->(i32, i32){
    let mut ans = (0, 0);
    let mut chars = line.chars().peekable();
    while let Some(ch) = chars.next(){
        match ch {
            'n' => {
                if let Some(x) = chars.peek() {
                    match x {
                        'e' => {ans.0 += 1;},
                        'w' => {ans.0 -= 1;},
                        _  => ()
                    } 
                    ans.1 += 1;
                };
                chars.next();
            },
            's' => {
                if let Some(x) = chars.peek() {
                    match x {
                        'e' => {ans.0 += 1;},
                        'w' => {ans.0 -= 1;},
                        _  => ()
                    } 
                    ans.1 -= 1;
                };
                chars.next();
            },
            'e' => {
                ans.0 += 2;
            },
            'w' => {
                ans.0 -= 2;
            },
            _ => ()
        };
    }
    ans
}

fn turns_black(tile: (i32, i32), seen: &HashSet<(i32,i32)>, directions: &Vec<(i32,i32)>)->bool{
    let is_black = seen.contains(&tile);
    
    let mut count = 0;
    for dir in directions{
        let neighbour = (tile.0 + dir.0, tile.1 + dir.1);
        if seen.contains(&neighbour){
            count += 1;
        }
    };
    if is_black {
        !((count == 0) | (count > 2))
    } else {
        count == 2
    }   
}

fn tick_day(black_tiles: &HashSet<(i32,i32)>, directions: &Vec<(i32,i32)>)->HashSet<(i32,i32)>{
    let mut n = -120;
    let mut new_black_tiles = HashSet::new();
    while n < 120 {
        let mut e = -150 - (n%2);
        while e < 150 + (n%2) {
            if turns_black((n,e), black_tiles, directions){
                new_black_tiles.insert((n,e));
            }
            e += 2;
        }
        n +=1;
    }
    new_black_tiles
}