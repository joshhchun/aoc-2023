use std::fs;


fn prediction(line: &str) -> i64 {
    let nums = line.split(" ").map(|n| n.parse().unwrap()).collect::<Vec<i64>>();
    
    let mut curr = nums;
    let mut res: i64 = 0;
    loop {
        if curr.iter().all(|x| x == &0) {
            break;
        }
        res += curr.last().unwrap();
        let mut new = Vec::new();
        for i in 0..curr.len()-1 {
            let diff: i64 = curr[i+1] - curr[i];
            new.push(diff);
        }
        curr = new;
    }
    return res;
}

fn main() {
    let res: i64 = fs::read_to_string("input.txt")
        .expect("should read file")
        .split("\n")
        .map(|l| prediction(l))
        .sum();
    println!("{:?}", res);

}
