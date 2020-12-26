use crate::utils::file2vec;

pub fn day25(filename:&String){
    let contents = file2vec::<i64>(filename);
    let contents:Vec<i64> =  contents.iter()
    .filter_map(|x| {
        match x {
            Ok(elt) => Some(*elt),
            _ => None
        }
    }
    )
    .collect();
    let door_loop_size = find_loop_size(contents[1], 7);
    let mut l = 0;
    let mut part1_ans = 1;
    while l < door_loop_size {
        part1_ans *= contents[0];
        part1_ans %= 20201227;
        l+=1;
    };
    println!("Part 1 ans: {}", part1_ans);

}

fn find_loop_size(target: i64, special_num:i64)-> i64{
    let mut ans: i64 = 0;
    let mut sn: i64 = 1;
    loop {
        sn *= special_num;
        sn %= 20201227;
        ans += 1;
        if sn == target {
            break
        }
    }
    ans
}