use crate::utils::file2vec;
use std::collections::{VecDeque, HashMap, HashSet};
pub fn day22(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();
    let mut player1 = VecDeque::new();
    let mut player2 = VecDeque::new();
    
    let mut p1 = true;
    for line in &contents[1..]{
        match line.chars().nth(0){
            Some(x) if x.is_numeric()=>{
                if p1 {
                    player1.push_back(line.parse::<i32>().unwrap());
                } else {
                    player2.push_back(line.parse::<i32>().unwrap());
                }
            },
            Some(x) if x == 'P' => {
                p1 = false;
            },
            _ => ()
        }
    }
    

    let part1_ans = combat(&mut player1.clone(), &mut player2.clone());
    println!("{:?}", part1_ans);
    println!("{:?}", player2);
    let part2_ans = recursive_combat(&mut player1, &mut player2);
    println!("{:?}", part2_ans);
}

fn deck_score(deck: &VecDeque<i32>)->i32{
    let mut score = 0;
    let mut idx = deck.len() as i32;
    for card in deck {
        score += *card * idx;
        idx -= 1;
    }
    score
}

fn turn(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>){
    let card1 = p1.pop_front().unwrap();
    let card2 = p2.pop_front().unwrap();
    if card1 > card2 {
        p1.push_back(card1);
        p1.push_back(card2);
    } else {
        p2.push_back(card2);
        p2.push_back(card1);
    }
}

fn combat(player1: &mut VecDeque<i32>, player2: &mut VecDeque<i32>)->GameResult{
    while !(player1.is_empty() | player2.is_empty()){
        turn(player1, player2);
    }
    let part1_ans = if player1.is_empty() {
        GameResult{ winner: Winner::Player2, cause: Cause::AllCards, score: deck_score(&player2)}
    } else {
        GameResult{ winner: Winner::Player1, cause: Cause::AllCards, score:deck_score(&player1)}
    };
    part1_ans
}

fn recursive_turn(p1: &mut VecDeque<i32>, p2: &mut VecDeque<i32>, seen: &mut HashSet<Vec<i32>>, level: usize)->Option<GameResult>{
    println!("Recursion level {}", level);
    if seen.contains(&to_key(p1, p2)){
        println!("This happened: {:?}", seen);
        return Some(GameResult{ winner: Winner::Player1, cause: Cause::Recursion, score: deck_score(p1)})
    };
    seen.insert(to_key(&p1, &p2));
    if p1.is_empty() {
        return Some(GameResult{ winner: Winner::Player2, cause: Cause::SubGame, score: deck_score(p2)})
    }
    if p2.is_empty() {
        return Some(GameResult{ winner: Winner::Player1, cause: Cause::SubGame, score: deck_score(p1)})
    }
    let card1 = p1.pop_front().unwrap();
    let card2 = p2.pop_front().unwrap();
    let mut res =None;
    if (p1.len() >= card1 as usize) & (p2.len() >= card2 as usize ){
        println!("card 1 {}, p1 len {}, card 2 {}, p2 len {}", card1, p1.len(), card2, p2.len());
        let mut new_p1 = p1.clone().drain(..card1 as usize).collect();
        let mut new_p2: VecDeque<i32> = p2.clone().drain(..card2 as usize).collect();
        // seen.insert(to_key(&new_p2));
        if let Some(r) = recursive_turn(&mut new_p1, &mut new_p2, seen, level+1){
            if r.cause == Cause::Recursion {
                res = Some(r);
            }
            match r.winner {
                Winner::Player1 => {
                    p1.push_back(card1);
                    p1.push_back(card2);
                },
                Winner::Player2 =>{
                    println!("{}-{}", card1, card2);
                    p2.push_back(card2);
                    p2.push_back(card1);
                }
            }
            println!("{:?}", to_key(p1, p2));
        }

    } else {
        if card1 > card2 {
            p1.push_back(card1);
            p1.push_back(card2);
        } else {
            p2.push_back(card2);
            p2.push_back(card1);
        }

    }

    res
}

fn to_key(v1: &VecDeque<i32>, v2: &VecDeque<i32>)->Vec<i32>{
    let mut out = Vec::new();
    for elt in v1{
        out.push(*elt);
    }
    for elt in v2{
        out.push(*elt);
    }
    out
}

fn recursive_combat(player1: &mut VecDeque<i32>, player2: &mut VecDeque<i32>)->GameResult{
    let mut seen = HashSet::new();
    let mut turns = 1;
    while !(player1.is_empty() | player2.is_empty()){
        println!("turn {}", turns);
        println!("Player 1: {:?}", player1);
        println!("Player 2: {:?}", player2);
        let res = recursive_turn(player1, player2, &mut seen, 0);
        println!("{:?}", res);
        if let Some(r) = res{
            if r.cause == Cause::Recursion {
                return  GameResult{ winner: Winner::Player1, cause: Cause::AllCards, score:deck_score(&player1) }
            }
        };
        turns +=1;
    }
    if player1.is_empty() {
        println!("{:?}", player2);
        GameResult{ winner: Winner::Player2, cause: Cause::AllCards, score: deck_score(&player2)}
    } else {
        GameResult{ winner: Winner::Player1, cause: Cause::AllCards, score:deck_score(&player1)}
    }
}

#[derive(Debug,Clone, Copy)]
enum Winner{
    Player1,
    Player2
}
#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Cause{
    Recursion,
    SubGame,
    AllCards
}

#[derive(Debug,Clone, Copy)]
struct GameResult{
    winner: Winner,
    cause: Cause,
    score: i32
}