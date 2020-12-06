use std::fs;
use std::str::FromStr;

pub fn file2vec<T: FromStr>(filename: &String)->Vec<Result<T, T::Err>>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.split("\n")
    .map(|x: &str| x.parse::<T>())
    .collect()
} 

// pub struct GappyList<'a, T>{
//     list: &'a Vec<String>,
//     ptr: usize,
//     size: usize,
//     parse: fn(&String)->T
// }


// impl<'a> Iterator for PassPortList<'a> {
//     type Item = PassPort;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.ptr {
//             x if x<self.size => {
//                 let mut map = HashMap::new();
//                 let mut row = self.passports[self.ptr].to_owned();
//                 while row != "" {
//                     map = parse_line(row, map);
//                     self.ptr += 1;
//                     if self.ptr == self.size {
//                         break
//                     }
//                     row = self.passports[self.ptr].to_owned();
//                 };
//                 self.ptr += 1;
//                 Some(PassPort::from_dict(map))
//             },
//             _ => None
//         }
        
//     }
// }