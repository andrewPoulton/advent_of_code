use std::collections::HashMap;
use crate::utils::file2vec;

pub fn day6(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:GappyList = GappyList{list: &contents.iter().map(|x| x.to_owned().unwrap()).collect(), ptr: 0, size:contents.len()};
    let ans: Vec<i32> = contents.into_iter()
    .fold(vec![0,0], |mut acc, group|{
        acc[0] += group.answers.len() as i32;
        acc[1] += group.answered_by_all();
        acc
    });
    println!("Part 1 ans: {}\nPart 2 ans: {}", ans[0], ans[1]);

    

}

struct GappyList<'a>{
    list: &'a Vec<String>,
    ptr: usize,
    size: usize,
    // parse: fn(&String)->T
}

struct GroupAnswer{
    answers: HashMap<char, usize>,
    size: usize
}
impl GroupAnswer {
    fn answered_by_all(&self)->i32{
        self.answers.values().fold(0, |mut acc, val|{
            acc += if *val == self.size {1} else {0};
            acc
        })
    }
}
fn parse_line(line: &String, map: HashMap<char, usize>)->HashMap<char, usize>{
    line.chars().fold(map, |mut acc, ch| -> HashMap<char, usize> {
        match acc.get_mut(&ch){
            Some(x) => *x+=1,
            None => {acc.insert(ch, 1); ()}
        };
        acc
    })
}

impl<'a> Iterator for GappyList<'a> {
    type Item = GroupAnswer;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            x if x < self.size => {
                let mut map = HashMap::new();
                let mut row = &self.list[self.ptr];
                let mut size:usize = 0;
                while row != "" {
                    map = parse_line(row, map);
                    size += 1;
                    self.ptr += 1;
                    if self.ptr == self.size {
                        break
                    }
                    row = &self.list[self.ptr];
                };
                self.ptr += 1;
                Some(GroupAnswer{ answers: map, size: size})
            },
            _ => {self.ptr = 0; None}
        }
        
    }
}