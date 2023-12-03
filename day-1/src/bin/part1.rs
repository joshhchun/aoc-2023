use std::fs;

fn calibration(line: &str) -> u32 {
    let chars = line.chars();
    let (mut first_num, mut sec_num) = ((0, chars.clone().count()), (0, 0));
    for (idx, ch) in chars.filter(|ch| ch.is_digit(10)).map(|ch| ch.to_digit(10)).enumerate() {
        if idx <= first_num.1 {
            first_num = (ch.unwrap(), idx);
        }
        if idx >= sec_num.1 {
            sec_num = (ch.unwrap(), idx);
        }
    }
    (first_num.0*10) + sec_num.0
}

fn main() {
    let res: u32 = fs::read_to_string("input.txt").unwrap().lines().map(calibration).sum();
    println!("{res}");
}
