use crate::utils::file2vec;

pub fn day20(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();
    let mut tiles = Vec::new();
    let mut tile:Vec<Vec<char>> = Vec::new();
    let mut id = String::from("");
    for line in &contents{
        if line.contains("Tile "){
            for ch in line.chars(){
                if ch.is_numeric(){
                    id.push(ch);
                };
            };
        } else if line.contains('#') | line.contains('.') {
            tile.push(line.to_owned().chars().collect());
        }
        else if id.len()>0{
            let new_tile = tile.drain(..).collect();
            
            tiles.push(Tile::new(id.parse::<i32>().unwrap(), new_tile));
            id = String::from("");
        }
    }
    
    let mut tile = Box::new(tiles[0].clone());
    let tile2 =  tiles[1].clone();

    tile.north = Some(Box::new(tile2));
    println!("{}", tile);
    tiles.swap_remove(0);
    // println!("{:?}", tile.north.unwrap().bottom_boarder());
    // let boarders = vec![&tile.bottom_boarder(), &tile.top_boarder(), &tile.left_boarder(), &tile.right_boarder()];
    let i = 0;
    while i < tiles.len(){
        
    }
}
#[derive(Debug, Clone)]
struct Tile{
    id: i32,
    tile: Vec<Vec<char>>,
    north: Option<Box<Tile>>,
    south: Option<Box<Tile>>,
    east: Option<Box<Tile>>,
    west: Option<Box<Tile>>,
}

use std::fmt;
impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}\n", self.id)?;
        for row in &self.tile{
            for ch in row {
                write!(f, "{} ", ch)?;
            }
            write!(f, "\n")?;
        };
        match &self.north {
            Some(x) => write!(f,"North:\n{}", x),
            None => write!(f, "")
        }
    }
}

impl Tile{
    fn new(id:i32, tile:Vec<Vec<char>>)->Self {
        Self { id, tile, north:None, south:None, east: None, west:None }
    }
    fn rotate(&mut self){
        //transpose then reverse
        let mut new_tiles = self.tile.clone();
        let cols = self.tile[0].len();
        for i in 0..self.tile.len(){
            for j in 0..cols{
                new_tiles[j][cols-i-1] = self.tile[i][j]
            }
        }
        self.tile = new_tiles;
    }

    fn flip_horizontally(&mut self){
        let mut new_tiles = self.tile.clone();
        let cols = self.tile[0].len();
        for i in 0..self.tile.len(){
            for j in 0..cols{
                new_tiles[i][cols-j-1] = self.tile[i][j]
            }
        }
        self.tile = new_tiles;
    }

    fn flip_vertically(&mut self){
        let mut new_tiles = self.tile.clone();
        let rows = self.tile.len();
        for i in 0..self.tile.len(){
            for j in 0..rows{
                new_tiles[rows-i-1][j] = self.tile[i][j]
            }
        }
        self.tile = new_tiles;
    }

    fn rotate_n(&mut self, n: usize){
        for _ in 0..n{
            self.rotate();
        }
    }

    fn top_boarder(&self)->Vec<usize>{
        self.tile
        .first()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i,ch)|{
            if ch == &'#'{
                Some(i)
            }else{
                None
            }
        }).collect()
    }

    fn bottom_boarder(&self)->Vec<usize>{
        self.tile
        .last()
        .unwrap()
        .iter()
        .enumerate()
        .filter_map(|(i,ch)|{
            if ch == &'#'{
                Some(i)
            }else{
                None
            }
        }).collect()
    }

    fn right_boarder(&self)->Vec<usize>{
        let mut boarder = Vec::new();
        for i in 0..self.tile.len(){
            if self.tile[i].last().unwrap() == &'#'{
                boarder.push(i);
            }
        }
        boarder
    }

    fn left_boarder(&self)->Vec<usize>{
        let mut boarder = Vec::new();
        for i in 0..self.tile.len(){
            if self.tile[i].first().unwrap() == &'#'{
                boarder.push(i);
            }
        }
        boarder
    }

    // fn in_top_row(&mut self, other: &Vec<Tile>)->bool{

    // }
}