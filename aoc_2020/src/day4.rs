use std::collections::HashMap;
use crate::utils::file2vec;

pub fn day4(filename: &String){
    let contents = file2vec::<String>(filename);
    let contents:PassPortList = PassPortList { passports : &contents.iter().map(|x| x.to_owned().unwrap()).collect(), 
                                                ptr:0, 
                                                size: contents.len()};

    let iter = contents.into_iter();
    let count = iter.fold(vec![0,0], |mut acc, pport|{
        acc[0] += if pport.is_valid1(){1}else{0};
        acc[1] += if pport.is_valid2(){1}else{0};
        acc
    });
    println!("{} valid passports pt1", count[0]);
    
    println!("{} valid passports pt2", count[1]);
}

struct PassPortList<'a> {
    passports: &'a Vec<String>,
    ptr: usize,
    size: usize

}
#[derive(Debug)]
struct PassPort {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl PassPort{
    fn from_dict(mut map: HashMap<String, String>)->PassPort{
        let byr:Option<String> = map.remove("byr");
        let iyr:Option<String> = map.remove("iyr");
        let eyr:Option<String> = map.remove("eyr");
        let hgt:Option<String> = map.remove("hgt");
        let hcl:Option<String> = map.remove("hcl");
        let ecl:Option<String> = map.remove("ecl");
        let pid:Option<String> = map.remove("pid");
        let cid:Option<String> = map.remove("cid");
        PassPort { byr, iyr, eyr, hgt, hcl, ecl, pid, cid}
    }

    fn is_valid1(&self)->bool{
        true & match &self.byr {
            Some(x) => true,
            None=> false
        } & match &self.iyr {
            Some(x) => true,
            None=> false
        } & match &self.eyr {
            Some(x) => true,
            None=> false
        } & match &self.hgt {
            Some(x) => true,
            None=> false
        } & match &self.hcl {
            Some(x) => true,
            None=> false
        } & match &self.ecl {
            Some(x) => true,
            None=> false
        } & match &self.pid {
            Some(x) => true,
            None=> false
        }

    }

    fn is_valid2(&self)->bool{
        true & match &self.byr {
            Some(x) => {
                (x.len() == 4) & match x.parse::<i32>() {
                    Ok(val) => (1920 <= val) & (val <= 2002),
                    Err(e)=>false
                }
            },
            None=> false
        } & match &self.iyr {
            Some(x) => {
                (x.len() == 4) & match x.parse::<i32>() {
                    Ok(val) => (2010 <= val) & (val <= 2020),
                    Err(e)=> false
                }
            },
            None=> false
        } & match &self.eyr {
            Some(x) => {
                (x.len() == 4) & match x.parse::<i32>() {
                    Ok(val) => (2020 <= val) & (val <= 2030),
                    Err(e)=>false
                }
            },
            None=> false
        } & match &self.hgt {
            Some(x) => {
                match &x[&x.len()-2..] {
                    "cm" => match &x[..&x.len()-2].parse::<i32>() {
                        Ok(val) => (150 <= *val) & (*val <= 193),
                        Err(e)=> false
                    },
                    "in" => match &x[..&x.len()-2].parse::<i32>() {
                        Ok(val) => (59 <= *val) & (*val <= 76),
                        Err(e)=> false
                    },
                    _ => false
                }
            },
            None=> false
        } & match &self.hcl {
            Some(x) => {
                (&x[0..1] == "#") & (x[..].len() == 7 as usize) & x[1..].chars().fold(true, |mut acc, c| {
                    acc &= c.is_digit(16);
                    acc
                })
            },
            None=> false
        } & match &self.ecl {
            Some(x) => {
                (x == "amb") | (x == "blu") | (x == "brn") | (x == "gry") | (x == "grn") | (x == "hzl") | (x == "oth")
            },
            None=> false
        } & match &self.pid {
            Some(x) => {
                (x.len() == 9) & x.chars().fold(true, |mut acc, c| {
                    acc &= c.is_digit(10);
                    acc
                })
            },
            None=> false
        }

    }
}

impl<'a> Iterator for PassPortList<'a> {
    type Item = PassPort;

    fn next(&mut self) -> Option<Self::Item> {
        match self.ptr {
            x if x<self.size => {
                let mut map = HashMap::new();
                let mut row = self.passports[self.ptr].to_owned();
                while row != "" {
                    map = parse_line(row, map);
                    self.ptr += 1;
                    if self.ptr == self.size {
                        break
                    }
                    row = self.passports[self.ptr].to_owned();
                };
                self.ptr += 1;
                Some(PassPort::from_dict(map))
            },
            _ => None
        }
        
    }
}

fn parse_line(line: String, mut map: HashMap<String, String>)->HashMap<String, String>{
    line.split_whitespace()
    .fold(map, |mut acc, field| -> HashMap<String, String> {
        let key_value: Vec<&str> = field.split(':').collect();
        acc.insert(key_value[0].to_owned(), key_value[1].to_owned());
        acc
    })
}