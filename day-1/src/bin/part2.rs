use std::fs;

fn calibration(line: &str, num_to_str: [&str; 9]) -> u32 {
    let chars = line.chars();
    let (mut first_num, mut sec_num) = ((0, chars.clone().count()), (0, 0));
    for (idx, num) in num_to_str.iter().enumerate() {
        if line.contains(num) {
            let str_idx = line.find(num).unwrap();
            if str_idx <= first_num.1 {
                first_num = (idx as u32 + 1, str_idx);
            }
            if str_idx >= sec_num.1 {
                sec_num = (idx as u32 +1, str_idx);
            }
        }
    }
    for (idx, ch) in chars.filter(|ch| ch.is_digit(10)).map(|ch| ch.to_digit(10)).enumerate() {
        if idx <= first_num.1 {
            first_num = (ch.unwrap(), idx);
        }
        if idx >= sec_num.1 {
            sec_num = (ch.unwrap(), idx);
        }
    }
    first_num.0*10 + sec_num.0
}

fn main() {
    let num_to_str = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let res: u32 = fs::read_to_string("input.txt").unwrap().lines().map(|line| calibration(line, num_to_str)).sum();
    println!("{res}");
}
