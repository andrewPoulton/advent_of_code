use crate::utils::file2vec;

pub fn day2(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();
    
    let valid_passwords: Vec<&String> = contents.iter().filter(|row| validate_row_part1(parse_row(row))).collect();
    println!("{} valid passwords part1", valid_passwords.len());
    let valid_passwords: Vec<&String> = contents.iter().filter(|row| validate_row_part2(parse_row(row))).collect();
    println!("{} valid passwords part2", valid_passwords.len());
}

#[derive(Debug)]
struct ParsedRow<'a>{
    min: usize,
    max: usize,
    letter: char,
    password: &'a str
}

fn parse_row(row: &String)->ParsedRow{
    let split_row: Vec<&str> = row.split_whitespace().collect();
    let limits:Vec<usize> = split_row[0].split("-").map(|x|x.parse::<usize>().unwrap()).collect();
    ParsedRow { min:limits[0], max:limits[1], letter:split_row[1].chars().nth(0).unwrap(), password:split_row[2]}
}

fn validate_row_part1(row:ParsedRow)->bool{
    let valid_chars:Vec<char> = row.password.chars()
    .filter(|c| c == &row.letter).collect();
    (row.min <= valid_chars.len()) & (valid_chars.len() <= row.max)
}

fn validate_row_part2(row:ParsedRow)->bool{
    let valid_chars:Vec<(usize,char)> = row.password.char_indices()
    .filter(|(idx, c)| 
    (c == &row.letter) & ((idx+1 == row.min) | (idx+1==row.max)))
    .collect();

    valid_chars.len()==1
}

