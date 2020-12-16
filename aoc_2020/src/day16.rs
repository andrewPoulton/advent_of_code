use std::collections::HashMap;

use crate::utils::file2vec;

pub fn day16(filename:&String){
    let contents = file2vec::<String>(filename);
    let contents:Vec<String> = contents.iter().map(|x| x.to_owned().unwrap()).collect();

    
    let mut i:usize = 0;
    let mut rules:Vec<Boundary> = Vec::new();
    let mut my_ticket:Vec<i32> = Vec::new();
    let mut nearby_tickets:Vec<Vec<i32>> = Vec::new();
    let mut line_type = LineType::Rule;
    while i < contents.len(){
        let line = &contents[i];
        if line.len() > 0 {
            match &line[..4] {
                "your" => {
                    line_type = LineType::MyTicket;
                    i+=1;
                },
                "near" => {
                    line_type = LineType::NearbyTickets;
                    i+=1;
                }
                _ => {
                        match line_type {
                            LineType::Rule => {
                                rules.push(parse_rule(line));
                            },
                            LineType::MyTicket => {
                                my_ticket = line.split(',').map(|x|x.to_owned().parse::<i32>().unwrap()).collect();
                            },
                            LineType::NearbyTickets => {
                                nearby_tickets.push(line.split(',').map(|x|x.to_owned().parse::<i32>().unwrap()).collect());
                            }
                        };
                        i+=1;
                }
            }
        } else {
            i+=1;
        }
    }
    let mut boundary = Boundary::new();
    for rule in &rules {
        boundary.lmin = boundary.lmin.min(rule.lmin);
        boundary.lmax = boundary.lmax.max(rule.lmax);
        boundary.rmin = boundary.rmin.min(rule.rmin);
        boundary.rmax = boundary.rmax.max(rule.rmax);
    }
    // boundary is now a `master rule`

    let mut sum = 0;
    let mut candidate_fields:Vec<HashMap<String, i32>> = vec![HashMap::new(); rules.len()]; 
    
    for ticket in &nearby_tickets {
        sum += ticket
        .iter()
        .filter(|x|{
            !boundary.valid(**x)
        }).sum::<i32>();
        
    }
    println!("part 1 ans: {}", sum);

    // get rid of invalid tickets for part 2
    let mut valid_tickets: Vec<&Vec<i32>> = nearby_tickets.iter().filter(|ticket| boundary.valid_ticket(*ticket)).collect();
    valid_tickets.push(&my_ticket);
    
    //part 2

    //start by checking how many times a ticket field is valid according to a specific rule
    for ticket in &valid_tickets {
        
        for (i, field) in ticket.iter().enumerate(){
            for rule in &rules{
                if let true = rule.valid(*field) {
                    let count = candidate_fields[i].entry(rule.name.to_owned()).or_insert(0);
                    *count += 1;
                }
            }
        }
        
    }

    //retain only those rules that are always accepted at given fields
    for candidate in &mut candidate_fields {
        candidate.retain(|_, v| *v == valid_tickets.len() as i32);
    }

    let mut fields = HashMap::new();
    for (i,candidate) in candidate_fields.iter().enumerate(){
        for field in candidate.keys(){
            let positions = fields.entry(field).or_insert(vec![0;rules.len()]);
            positions[i] += 1;
        }
    }


    let mut potential_fields: Vec<(&String, Vec<i32>)> = fields.drain().collect();

    // sort potentials by how many potential fields the correspond to
    potential_fields.sort_by(|a, b| {
        a.1.iter().sum::<i32>().partial_cmp(&b.1.iter().sum::<i32>()).unwrap()
        
    }
    );

    let mut current_fields = vec![0; rules.len()];
    let mut part2_ans:i64 = 1;
    for (k,v) in potential_fields{
        for i in 0..current_fields.len(){
            //if current_fields differs from v at i, this is the unique field k can correspond to
            if let 1 = current_fields[i] ^ v[i]{
                match &k[..3] {
                    "dep" => {
                        part2_ans *= my_ticket[i] as i64;
                        
                    },
                    _ => ()
                }
            }
            current_fields[i] = v[i];
        }
    }
    println!("part 2 ans: {}", part2_ans);
}

#[derive(Debug)]
enum LineType{
    Rule,
    MyTicket,
    NearbyTickets
}

#[derive(Debug)]
struct Boundary {
    lmin :i32,
    lmax: i32,
    rmin: i32,
    rmax: i32,
    name: String
}

impl Boundary {
    fn new()-> Boundary {
        Boundary { lmin: std::i32::MAX, lmax: std::i32::MIN, rmin: std::i32::MAX, rmax: std::i32::MIN, name: String::from("Overall") }
    }

    fn valid(&self, x: i32) -> bool {
        ((x>=self.lmin) & (x <= self.lmax)) | ((x>=self.rmin) & (x<=self.rmax))
    }

    fn valid_ticket(&self, ticket: &Vec<i32>)->bool{
        for field in ticket{
            if !self.valid(*field) {
                return false
            }
        }
        true
    }
}

fn parse_rule(s: &str)->Boundary{
    let mut name = String::from("");
    let boundaries: Option<&str> = if let Some(idx) = s.find(':'){
        name = s[..idx].to_owned();
        Some(s[idx..].trim())
    } else {None};
    let mut res = Vec::new();
    if let Some(b) = boundaries {
        let mut val = String::from("");
        for ch in b.chars(){
            if ch.is_numeric(){
                val.push(ch);
            } else if val.len()>0 {
                res.push(val.parse::<i32>().unwrap());
                val = String::from("");
            }
        }
        if val.len()>0 {
            res.push(val.parse::<i32>().unwrap());
        }
    }
    Boundary { lmin :res[0], lmax: res[1], rmin: res[2], rmax: res[3], name: name}
}