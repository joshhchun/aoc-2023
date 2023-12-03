use std::{fs, cmp};
use std::collections::HashMap;

fn calc_min(line: Vec<&str>) -> u32 {
    let mut min_dice: HashMap<&str, u32> = HashMap::new();

    for game in line[1].split("; ") {
        for pull in game.split(", ") {
            let x: Vec<&str> = pull.split(" ").collect();
            let (num_dice, color_dice) = (x[0].parse::<u32>().unwrap(), x[1]);

            match min_dice.contains_key(color_dice) {
               false => min_dice.insert(color_dice, num_dice),
               true =>  min_dice.insert(color_dice, cmp::max(min_dice[color_dice], num_dice))
            };
        }
    }
    
    let mut res = 1;
    for (_, val) in &min_dice {
        res *= val;
    }
    return res;
}

fn main() {
    let res: u32 = fs::read_to_string("input.txt")
        .expect("should have read the file")
        .lines()
        .map(|line| calc_min(line.split(": ").collect::<Vec<&str>>()))
        .sum();

    println!("{res}");
}
