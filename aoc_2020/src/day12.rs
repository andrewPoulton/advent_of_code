use crate::utils::file2vec;

pub fn day12(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    
    let mut pos = Position::start();
    
    for act in &contents {
        let action = Action::from_str(&act);
        pos.take_action(&action);
    }
    let dist = pos.manhattan();
    println!("part 1 ans: {}", dist);

    pos = Position::start();
    for act in &contents {
        let action = Action::from_str(&act);
        pos.take_action_with_waypoint(&action);
    }
    let dist = pos.manhattan();
    println!("part 2 ans: {}", dist);

    
}
#[derive(Debug, Copy, Clone)]
struct Position {
    n: i32,
    e: i32,
    s: i32,
    w: i32,
    direction: char,
    waypoint: WayPoint
}

#[derive(Debug,Copy, Clone)]
struct WayPoint {
    n: i32,
    e: i32
}
#[derive(Debug, Copy, Clone)]
struct Action {
    act: char,
    val: i32
}

impl Action {
    fn from_str(s:&str)-> Action{
        let act = s.chars().next().unwrap();
        let val = s[1..].to_string().parse::<i32>().unwrap();
        Action { act, val }
    }
}

impl Position {
    fn start()->Position{
        Position { n: 0, e: 0, s: 0, w: 0, direction: 'E', waypoint: WayPoint{ n:1, e: 10} }
    }

    fn take_action(&mut self, action: &Action){
        match action.act {
            'N' => {self.n += action.val;},
            'S' => {self.s += action.val;},
            'E' => {self.e += action.val;},
            'W' => {self.w += action.val;},
            'L' => {
                let turns = (action.val / 90) % 4;
                let directions = ['N','W','S','E'];
                if let Some(idx) = directions.iter().
                position(|ch| ch == &self.direction) {
                    self.direction = directions[(idx + turns as usize) % 4];
                };
            },
            'R' => {
                let turns = (action.val / 90) % 4;
                let directions = ['N','E','S','W'];
                if let Some(idx) = directions.iter().
                position(|ch| ch == &self.direction) {
                    self.direction = directions[(idx + turns as usize) % 4];
                };
            },
            'F' => {
                let act = Action{act:self.direction, val: action.val};
                self.take_action(&act);
            },
            _ => ()
        }
    }

    fn take_action_with_waypoint(&mut self, action: &Action){
        match action.act {
            'N' => {self.waypoint.n += action.val;},
            'S' => {self.waypoint.n -= action.val;},
            'E' => {self.waypoint.e += action.val;},
            'W' => {self.waypoint.e -= action.val;},
            'L' => {
                let turns = (action.val / 90) % 4;
                for _ in 0..turns {
                    let tmp_n = self.waypoint.e;
                    self.waypoint.e = -self.waypoint.n;
                    self.waypoint.n = tmp_n;
                };
            },
            'R' => {
                let turns = (action.val / 90) % 4;
                for _ in 0..turns {
                    let tmp_n = -self.waypoint.e;
                    self.waypoint.e = self.waypoint.n;
                    self.waypoint.n = tmp_n;
                };
            },
            'F' => {
                let n = self.waypoint.n * action.val;
                let w = self.waypoint.e * action.val;
                if n < 0 {self.s -= n} else {self.n += n};
                if w < 0 {self.w -= w} else {self.e += w};
            },
            _ => ()
        }
    }

    fn manhattan(&self)-> i32 {
        (self.n - self.s).abs() + (self.e - self.w).abs()
    }
}

