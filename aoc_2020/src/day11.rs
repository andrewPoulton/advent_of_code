use crate::utils::file2vec;

pub fn day11(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<Vec<Seat>> = contents.iter().map(|x| 
        x.to_owned().unwrap().chars().fold(Vec::new(), |mut acc, c| {
            acc.push(Seat::from_char(&c));
            acc
        })
    ).collect();
    let mut ferry = Ferry::new(contents);
    let mut loops = 0;
    loop {
        loops +=1;
        ferry.tick();
        if ferry.diff(){
            break
        };
        ferry.update();
    }
    println!("part 1 ans {}", ferry.count_occupied_seats());
    ferry.reset();

    let mut loops = 0;
    loop {
        loops +=1;
        ferry.tick_part2();

        if ferry.diff(){
            break
        };
        ferry.update();
    }
    println!("part 2 ans {}", ferry.count_occupied_seats());

}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum State {
    Occupied = 1,
    Unnocupied = 0,
    Floor = -1
}
#[derive(Debug, Copy, Clone)]
struct Seat {
    state: State,
    next_state: Option<State>
}

impl Seat {
    fn from_char(ch: &char)->Self{
        let state = match ch {
            'L' => State::Unnocupied,
            '#' => State::Occupied,
            _ => State::Floor
        };
        Seat { state, next_state: None } 
    }
}

#[derive(Debug)]
struct Ferry {
    seats: Vec<Vec<Seat>>,
    h: usize,
    w: usize
}

impl Ferry {
    fn new(seats: Vec<Vec<Seat>>)->Ferry {
        let h = seats.len();
        let w = seats[0].len();
        Ferry { seats, h, w }
    }

    fn occupied_neighbours(&mut self, row: usize, col: usize)->i8 {
        let mut count = 0;
        for delta_row in [-1, 0, 1].iter().cloned() {
            for delta_col in [-1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row = row as i16 + delta_row ;
                let neighbor_col = col as i16 + delta_col ;
                if (neighbor_col< self.w as i16) & (neighbor_col>=0) & (neighbor_row< self.h as i16) & (neighbor_row>=0){
                    count += 0.max(self.seats[neighbor_row as usize][neighbor_col as usize].state as i8);
                }
            }
        }
        count
    }

    fn diff(&self)->bool {
        for row in 0..self.h {
            for col in 0..self.w {
                if let Some(x) = self.seats[row][col].next_state {
                    return false
                }
            }
        };
        true
    }

    fn tick(&mut self){
        for row in 0..self.h{
            for col in 0..self.w{
                match self.seats[row][col].state {
                    State::Occupied => {
                        if &self.occupied_neighbours(row, col) >= &4 {
                            self.seats[row][col].next_state = Some(State::Unnocupied);
                        }
                    },
                    State::Unnocupied => {
                        if &self.occupied_neighbours(row, col) == &0 {
                            self.seats[row][col].next_state = Some(State::Occupied);
                        }
                    },
                    _ => ()
                };
            }
        }
    }

    fn update(&mut self){
        for row in 0..self.h{
            for col in 0..self.w{
                if let Some(state) = self.seats[row][col].next_state {
                    self.seats[row][col].state = state;
                    self.seats[row][col].next_state = None;
                }
            }
        }
    }

    fn count_occupied_seats(&self)->i16 {
        let mut count: i16 = 0;
        for row in 0..self.h{
            for col in 0..self.w{
                if self.seats[row][col].state == State::Occupied {
                    count +=1;
                }
            }
        }
        count
    }

    fn occupied_neighbours_part2(&mut self, row: usize, col: usize)->i8 {
        let mut count = 0;
        let mut look_row = row ;
        let mut look_col = col;

        //look right
        look_col += 1;
        while look_col < self.w {
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => {look_col += 1;},
                _ => {
                    
                    break
                }
            }
        }
        look_col = col;
        //look left
        // look_col -= 1;
        while look_col > 0 {
            look_col -= 1;
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => (),
                _ => {
                    break
                }
            }
        }
        look_col = col;

        //look down
        look_row += 1;
        while look_row < self.h {
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => {look_row += 1;},
                _ => {
                    break
                }
            }
        }
        look_row = row;

         //look up
         while look_row > 0 {
             look_row -= 1;
             match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => (),
                _ => {
                    break
                }
            }
        }
        look_row = row;

        //look down & right
        look_row += 1;
        look_col += 1;
        while (look_col < self.w) & (look_row < self.h){
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => {
                    look_row += 1;
                    look_col += 1;
                },
                _ => {
                    break
                }
            }
        }
        look_row = row;
        look_col = col;

        //look down & left
        look_row += 1;
        while (look_col >0) & (look_row < self.h){
            look_col -= 1;
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => {
                    look_row += 1;
                },
                _ => {
                    break
                }
            }
        }
        look_row = row;
        look_col = col;

        //look up & right
        look_col += 1;
        while (look_col < self.w) & (look_row > 0){
            look_row -= 1;
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => {
                    // look_row -= 1;
                    look_col += 1;
                },
                _ => {
                    break
                }
            }
        }
        look_row = row;
        look_col = col;
        
        //look up & left
        while (look_col > 0) & (look_row > 0){
            look_row -= 1;
            look_col -= 1;
            match self.seats[look_row][look_col].state {
                State::Occupied => {
                    count +=1;
                    break
                },
                State::Floor => (),
                _ => {
                    break
                }
            }
        }

        count
    }

    fn tick_part2(&mut self){
        for row in 0..self.h{
            for col in 0..self.w{
                
                match self.seats[row][col].state {
                    State::Occupied => {
                        if &self.occupied_neighbours_part2(row, col) >= &5 {
                            self.seats[row][col].next_state = Some(State::Unnocupied);
                        }
                    },
                    State::Unnocupied => {
                        if &self.occupied_neighbours_part2(row, col) == &0 {
                            self.seats[row][col].next_state = Some(State::Occupied);
                        }
                    },
                    _ => ()
                };
            }
        }
    }

    fn reset(&mut self){
        for row in 0..self.h{
            for col in 0..self.w{
                self.seats[row][col].next_state = None;
                match self.seats[row][col].state {
                    State::Occupied => {
                        self.seats[row][col].state = State::Unnocupied;
                    }
                    _ => ()
                }
            }
        }
    }
}