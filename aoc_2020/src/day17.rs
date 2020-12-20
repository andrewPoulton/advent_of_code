use std::collections::VecDeque;
use crate::utils::file2vec;

pub fn day17(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();

    // let mut z_index: usize = 0; //keep track of  
    let mut pocket_dimension:VecDeque<ZPlane> = VecDeque::from(vec![ZPlane::init(&contents)]);
    println!("intitial:\n{}", pocket_dimension[0]);
    let x = pocket_dimension[0].h + 20;
    pocket_dimension[0].expand_to(x);
    for _ in 0..8{
        pocket_dimension.push_front(ZPlane::blank(x));
        pocket_dimension.push_back(ZPlane::blank(x));
    }

    let mut part2_dimensions = VecDeque::from(vec![pocket_dimension.clone()]);
    for _ in 0..8{
        let mut front_dim = VecDeque::new();
        let mut back_dim = VecDeque::new();
        for _ in 0..pocket_dimension.len(){
            front_dim.push_back(ZPlane::blank(x));
            back_dim.push_back(ZPlane::blank(x));
        }
        part2_dimensions.push_back(back_dim);
        part2_dimensions.push_front(front_dim);
    }


    // println!("{}", pocket_dimension[1]);
    // println!("{}")
    // println!("{:?}", pocket_dimension[0].cubes[2][5]);
    // println!("{}-{}", pocket_dimension[0].h, pocket_dimension[0].w);
    // println!("{}", pocket_dimension[0].active_neighours(-1, 0, false));
    
    for _ in 0..6{
        pocket_dimension = tick(&pocket_dimension);
    }
    for pocket in &pocket_dimension{
        println!("{}", pocket);
    }
    let part_1_ans = pocket_dimension.iter().map(|plane| plane.count_active()).sum::<i32>();
    println!("{}", part_1_ans);
    // let mut i = -1;
    // for dim in &pocket_dimension{
    //     println!("Z = {}", i);
    //     println!("{}\n\n", dim);
    //     i +=1 ;
    // }
}

fn tick(pocket: &VecDeque<ZPlane>)->VecDeque<ZPlane>{
    let mut i: usize = 1;
    let mut out_plane = VecDeque::new();
    out_plane.push_back(pocket[0].clone());
    while i< pocket.len()-1{
        let prev_plane = &pocket[i-1];
        let plane = &pocket[i];
        let next_plane = &pocket[i+1];
        let mut new_plane = pocket[i].clone();
        for i in 1..new_plane.cubes.len()-1{
            for j in 1..new_plane.cubes[0].len()-1{
                let mut neighbours: u8 = 0;
                neighbours += plane.active_neighours(i as i32, j as i32, false);
                neighbours += prev_plane.active_neighours(i as i32, j as i32, true);
                neighbours += next_plane.active_neighours(i as i32, j as i32, true);
                new_plane.cubes[i][j] = State::from_count(&plane.cubes[i][j], neighbours);
            }
        }
        out_plane.push_back(new_plane);
        i+=1;
    }
    out_plane.push_back(pocket[pocket.len()-1].clone());
    out_plane
    // *pocket
}
// fn tick_part2(pocket: &VecDeque<VecDeque<ZPlane>>)->VecDeque<VecDeque<ZPlane>>{
//     let mut i: usize = 1;
//     let mut out_plane = VecDeque::from(vec![VecDeque::new()]);
//     out_plane.push_back(pocket[0].clone());
//     while i< pocket.len()-1{
//         let prev_plane = &pocket[i-1];
//         let plane = &pocket[i];
//         let next_plane = &pocket[i+1];
//         let mut new_plane = pocket[i].clone();
//         for i in 1..new_plane.cubes.len()-1{
//             for j in 1..new_plane.cubes[0].len()-1{
//                 let mut neighbours: u8 = 0;
//                 neighbours += plane.active_neighours(i as i32, j as i32, false);
//                 neighbours += prev_plane.active_neighours(i as i32, j as i32, true);
//                 neighbours += next_plane.active_neighours(i as i32, j as i32, true);
//                 new_plane.cubes[i][j] = State::from_count(&plane.cubes[i][j], neighbours);
//             }
//         }
//         out_plane.push_back(new_plane);
//         i+=1;
//     }
//     out_plane.push_back(pocket[pocket.len()-1].clone());
//     out_plane
//     // *pocket
// }
// fn neighbour_count(pocket: &VecDeque<ZPlane>, x:i32, y:i32, z:i32)->State{
//     todo!();
// }

#[derive(Debug, Clone)]
struct ZPlane{
    h: usize,
    w: usize,
    cubes: VecDeque<VecDeque<State>>
}

use std::fmt::Display;
impl Display for ZPlane {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cubes{
            for cube in row{
                match cube {
                    State::Active => write!(f, "#")?,
                    State::Inactive => write!(f, ".")?
                }
            }
            write!(f,"\n")?;
        }
        write!(f, "\n")
    }
}


#[derive(Debug,Copy, Clone)]
enum State{
    Active = 1 , 
    Inactive = 0, 
}

impl ZPlane {
    fn init(v: &Vec<String>)->ZPlane{
        let mut cubes = VecDeque::new();
        for (y, line) in v.iter().enumerate() {
            let mut row = VecDeque::new();
            for (x,ch) in line.char_indices(){
                if let Ok(cube) = State::from_char(&ch){
                    row.push_back(cube);
                }
            }
            if row.len()>0{
                cubes.push_back(row);
            }
        }
        ZPlane { h: cubes.len(), w: cubes[0].len(), cubes: cubes}
    }

    fn expand_to(&mut self, side_length: usize){
        let diff = (side_length - self.cubes.len()) / 2;
        for row in self.cubes.iter_mut(){
            for _ in 0..diff{
                row.push_front(State::Inactive);
                row.push_back(State::Inactive);
            }
        }
        for _ in 0..diff{
            self.cubes.push_front(VecDeque::from(vec![State::Inactive; self.cubes[0].len()]));
            self.cubes.push_back(VecDeque::from(vec![State::Inactive; self.cubes[0].len()]));
        }
        self.w = self.cubes[0].len();
        self.h = self.cubes.len();
    }

    fn blank(side_length: usize)->ZPlane{
        let mut cubes = VecDeque::new();
        for i in 0..side_length{
            let mut row = VecDeque::new();
            for j in 0..side_length{
                row.push_back(State::Inactive)
            }
            cubes.push_back(row);
        }
        ZPlane{ h: side_length, w:side_length, cubes}
    }

    fn active_neighours(&self, i: i32, j:i32, and_self: bool)-> u8{
        let mut count:u8 = 0;
        let h = self.h as i32;
        let w = self.w as i32;
        let x_delta = vec![-1, 0, 1];
        let y_delta = vec![-1, 0, 1];
        for dx in &x_delta{
            for dy in &y_delta{
                if (dy != &0) | (dx != &0){
                    let x = j+ *dx;
                    let y = i+ *dy;
                    if (0<=x) & (x<w) & (0<=y) & (y<h) {
                        // println!("{}-{}-{}-{}", y, x, w, h);
                        count += self.cubes[y as usize][x as usize] as u8;
                    }
                }
            }
        }
        if (and_self) & (i>=0) & (i<h) & (j>=0) & (j<w) {
            count += self.cubes[i as usize][j as usize] as u8;
            count
        } else {
            count
        }
    }

    fn count_active(&self)->i32{
        let mut count = 0;
        for row in &self.cubes{
            for cube in row{
                count += *cube as i32;
            }
        }
        count
    }
}

impl State{
    fn from_char(ch: &char)-> Result<State, &str>{
        match ch {
            '#' => Ok(State::Active),
            '.' => Ok(State::Inactive),
            _ => Err("invalid character")
        }
    }

    fn from_count(state: &State, count: u8)-> State{
        match *state {
            State::Active => {
                match count {
                    2 | 3 => State::Active,
                    _ => State::Inactive
                }
            },
            State::Inactive => {
                match count {
                    3 =>  State::Active,
                    _ => State::Inactive
                }
            }
        }
    }

}