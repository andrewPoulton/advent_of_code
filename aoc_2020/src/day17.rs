use std::collections::VecDeque;
use crate::utils::file2vec;

pub fn day17(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> =  contents.iter().map(|x| x.to_owned().unwrap()).collect();

    // let mut z_index: usize = 0; //keep track of  
    let mut pocket_dimension:VecDeque<ZPlane> = VecDeque::from(vec![ZPlane::init(&contents)]);


    println!("{:?}", pocket_dimension);
    // println!("{:?}", pocket_dimension[0].cubes[2][5]);
    println!("{}-{}", pocket_dimension[0].h, pocket_dimension[0].w);
    println!("{}", pocket_dimension[0].active_neighours(-1, 0, false));

    for _ in 0..6{
        pocket_dimension = tick(&pocket_dimension);
    }
    let part_1_ans = pocket_dimension.iter().map(|plane| plane.count_active()).sum::<i32>();
    println!("{}", part_1_ans);
}

fn tick(pocket: &VecDeque<ZPlane>)->VecDeque<ZPlane>{
    let mut ptr: i32 = -1;
    let mut new_pocket: VecDeque<ZPlane> = VecDeque::new();
    while ptr <= pocket.len() as i32{
        let mut y = 0;
        let mut x = 0;
        let plane = if (ptr < pocket.len() as i32) & (ptr >=0) {
            &pocket[ptr as usize]
        } else if ptr >= 0{
            &pocket[ptr as usize-1]
        } else {
            &pocket[0]
        };
        let mut new_plane = plane.clone();
        new_plane.h += 1;
        new_plane.w += 1;
        
        // iterate through existing rows, appending to end
        while y < plane.h as i32{
            while x <= plane.w as i32 {
                let new_cube = neighbour_count(pocket, x, y, ptr );
                if x == plane.w as i32{
                    new_plane.cubes[y as usize].push_back(new_cube)
                } else{
                    new_plane.cubes[y as usize][x as usize] = new_cube
                }
                x+=1;
            }
            y+=1;
        }
        x = 0;
        y = plane.h as i32;
        //add new last row
        let mut last_row = VecDeque::new();
        while x< plane.w as i32 {
            let new_cube = neighbour_count(pocket, x, y, ptr);
                last_row.push_back(new_cube);
            x+=1;
        }
        new_plane.cubes.push_back(last_row);

        // add new first row
        y = -1;
        x = 0;
        let mut first_row = VecDeque::new();
        while x< plane.w as i32 {
            let new_cube = neighbour_count(pocket, x, y, ptr);
            first_row.push_back(new_cube);
            x+=1;
        }
        new_plane.cubes.push_front(first_row);

        //add new first col
        for row in &mut new_plane.cubes {
            let new_cube = neighbour_count(pocket, x, y, ptr);
            row.push_front(new_cube);
        }
        new_pocket.push_back(new_plane);
        ptr += 1;
    }
    new_pocket
}

fn neighbour_count(pocket: &VecDeque<ZPlane>, x:i32, y:i32, z:i32)->State{
    let include_self = (z < pocket.len() as i32) & (z>=0);
    let plane = if include_self {
        &pocket[z as usize]
    } else if z == -1{
        &pocket[0]
    } else {
        &pocket[pocket.len()-1]
    };
    let h = plane.h as i32;
    let w = plane.w as i32;
    let mut neighbours = 0;
    neighbours += plane.active_neighours(y,x, include_self);
    if z > 0 {
        neighbours += pocket[z as usize -1].active_neighours(y,x, true);
    }
    if z < pocket.len() as i32 -1 {
        // println!("z is {}", z);
        neighbours += pocket[if z==-1{0}else{z as usize +1}].active_neighours(y,x, true);
    }
    let curr_state = if (x<=0) | (x>= plane.w as i32) | (y<=0) | (y>=plane.h as i32){
        &State::Inactive
    } else {
        // println!("{}-{}-{}", y,x,z);
        &plane.cubes[y as usize][x as usize]
    };
    State::from_count(curr_state, neighbours)
}

#[derive(Debug, Clone)]
struct ZPlane{
    h: usize,
    w: usize,
    cubes: VecDeque<VecDeque<State>>
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
                        println!("{}-{}-{}-{}", y, x, w, h);
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