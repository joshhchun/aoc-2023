use std::fs;

fn parse_num(content: &Vec<Vec<char>>, line_idx: usize, chr_idx: usize) -> i32 {
    // Go to start of num
    let mut chr_idx = chr_idx as i32;
    while chr_idx - 1 >= 0 && content[line_idx][(chr_idx - 1) as usize].is_numeric() {
        chr_idx -= 1;
    }

    let mut res = 0;
    let mut chr_idx = chr_idx as usize;
    while chr_idx < content[0].len() && content[line_idx][chr_idx].is_numeric() {
        res = (res * 10) + content[line_idx][chr_idx].to_digit(10).unwrap();
        chr_idx += 1;
    }
    
    res as i32
}

fn valid_check(new_line_idx: i32, new_chr_idx: i32, row_l: i32, col_l: i32) -> bool {
    new_line_idx >= 0 && new_line_idx < row_l
        && new_chr_idx >= 0 && new_chr_idx < col_l
}

fn check_box(content: &Vec<Vec<char>>, line_idx: usize, chr_idx: usize) -> i32 {
    let mut nums: Vec<i32> = Vec::new();

    for i in -1..2 {
        for j in -1..2 {
            let new_line_idx = line_idx as i32 + i;
            let new_chr_idx  = chr_idx  as i32 + j;
            valid_check(new_line_idx, new_chr_idx, content.len() as i32, content[0].len() as i32);
            let (new_line_idx, new_chr_idx) = (new_line_idx as usize, new_chr_idx as usize);
            if content[new_line_idx][new_chr_idx].is_numeric() {
                let num = parse_num(&content, new_line_idx, new_chr_idx);
                if !nums.iter().any(|&n| n == num) {
                    nums.push(num);
                }
            }
        }
    }
    nums.iter().sum()
}

fn main() {
    let content: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .expect("should read file")
        .lines().map(|line| line.chars().collect()).collect();

    let mut res = 0;

    for line_idx in 0..content.len() {
        for chr_idx in 0..content[line_idx].len() {
            match content[line_idx][chr_idx] {
                '.'      => continue,
                '0'..='9' => continue,
                _ => {
                    res += check_box(&content, line_idx, chr_idx);
                }
            }
        }
    }

    println!("{res}");
}
