use crate::utils::file2vec;

pub fn day5(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    let max_id = contents.iter().fold(0, |mut max, boarding_pass|{
        let id = find_seat(boarding_pass).id();
        max = if id > max {id} else {max};
        max
    });
    println!("max id is {}", max_id);
    let ids: Vec<i32> = contents.iter().fold(vec![0;8*128],|mut acc, bp|{
        acc[find_seat(bp).id() as usize] = 1;
        acc
    });
    let mut my_id = 0;
    let mut idx:usize  = 1;
    for elt in ids[1..].iter(){
        if (*elt == 0) & (ids[idx-1] == 1) & (ids[idx+1] == 1){
            my_id = idx;
            break
        };
        idx += 1;
    };
    println!("My seat is {}", my_id);
    


    
}
#[derive(Debug)]
struct Seat{
    row: i32,
    col: i32
}

impl Seat {
    fn id(&self)->i32{
        self.row * 8 + self.col
    }
}

fn find_seat(boarding_pass: &String)-> Seat{
    let mut row_l = 0;
    let mut row_r = 127;
    let mut col_l = 0;
    let mut col_r = 7;
    let mut row = 0;
    let mut col = 0;
    for ch in boarding_pass.chars(){
        match ch {
            'F' => {
                row_r = (row_l + row_r) >>1;
                row = row_r;
            },
            'B' => {
                row_l = (row_l + row_r) >>1;
                row = row_l+1;
            },
            'R' => {
                col_l = (col_l+col_r)>>1;
                col = col_l +1;
            },
            'L' => {
                col_r = (col_l+col_r)>>1;
                col = col_r;

            },
            _ => ()
        }
    };
    Seat{ row: row, col: col}
}