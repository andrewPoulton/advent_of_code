use std::fs;
use std::str::FromStr;

pub fn file2vec<T: FromStr>(filename: &String)->Vec<Result<T, T::Err>>{
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents.split("\n")
    .map(|x: &str| x.parse::<T>())
    .collect()
} 