use std::fs;
use std::collections::HashMap;


fn is_possible(line: Vec<&str>) -> u32 {
    let dice_num: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let game_id = line[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
    let games = line[1];
    for game in games.split("; ") {
        for pull in game.split(", ") {
            let x: Vec<&str> = pull.split(" ").collect();
            if x[0].parse::<u32>().unwrap() > dice_num[x[1]] {
                return 0;
            }
        }
    }
    return game_id;
}

fn main() {
    let res: u32 = fs::read_to_string("part1_test_input.txt")
        .expect("should have read the file")
        .lines()
        .map(|line| is_possible(line.split(": ").collect::<Vec<&str>>()))
        .sum();

    println!("{res}");
}
