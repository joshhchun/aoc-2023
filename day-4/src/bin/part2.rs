use std::{fs, collections::HashMap};

fn calc_wins(games: &str) -> u32 {
    let nums: Vec<Vec<&str>> = games.split(" | ")
        .map(|games| games.split(" ").collect())
        .collect();

    let (win_nums, elf_nums) = (&nums[0], &nums[1]);
    let mut matching: u32 = 0;

    for &val in elf_nums {
        if val != "" && win_nums.contains(&val) {
            matching += 1
        }
    }
    return matching;
}

fn main() {
    let mut scorecards: HashMap<u32, u32> = HashMap::new();
    let num_cards: u32 = fs::read_to_string("input.txt")
        .expect("should read file")
        .lines()
        .enumerate()
        .map(|(card_id, line)| {
            let val = calc_wins(&line.split(": ").collect::<Vec<&str>>()[1]);
            (card_id as u32 + 1 .. card_id as u32 + 1 + val as u32)
                .for_each(|copy_id| {
                    let add_val: &u32 = scorecards.entry(card_id as u32).or_insert(1);
                    *scorecards.entry(copy_id).or_insert(1) += *add_val;
                });
            scorecards[&(card_id as u32)]
        })
        .sum();
    println!("{num_cards}");
}

