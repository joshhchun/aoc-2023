use std::fs;

fn calc_wins(line: &str) -> u32 {
    let nums: Vec<Vec<&str>> = line.split(" | ")
        .map(|games| games.split(" ").collect())
        .collect();
    let (win_nums, elf_nums) = (&nums[0], &nums[1]);
    let mut res: u32 = 0;

    for &val in elf_nums {
        if val != "" && win_nums.contains(&val) {
            match res {
                0 => res = 1,
                _ => res *= 2
            }
        }
    }
    res
}

fn main() {
    let res: u32 = fs::read_to_string("input.txt")
        .expect("should read file")
        .lines()
        .map(|line| calc_wins(line.split(": ").collect::<Vec<&str>>()[1]))
        .sum();
    
    println!("{res}");
}
