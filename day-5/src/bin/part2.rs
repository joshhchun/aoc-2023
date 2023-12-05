use std::{fs, vec};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct RangeMapping {
    range_start: u64,
    range_end: u64,
    dest: u64,
}

impl RangeMapping {
    pub fn in_range(&self, val: &u64) -> bool {
        return *val >= self.range_start && *val <= self.range_end;
    }
}

fn parse_map(input: &str) -> Vec::<RangeMapping>{
    let mut arr: Vec::<RangeMapping> = vec!();
    input.split("\n")
        .skip(1)
        .for_each(|line| {
            let values: Vec<u64> = line.split(" ").map(|ele| ele.parse::<u64>().unwrap()).collect();
            let (map, start, len) = (values[0], values[1], values[2]);
            arr.push(RangeMapping{range_start: start, range_end: start+len, dest:map});
        });
    return arr;
}

fn get_num(curr_val: u64, mapping: &Vec<RangeMapping>) -> u64 {
    for range in mapping {
        if RangeMapping::in_range(&range, &curr_val) {
            return range.dest + (curr_val - range.range_start);
        }
    }
    return curr_val;
}

fn get_seeds(seeds: &str) -> Vec<u64> {
    let mut seeds: Vec<(u64, u64)> = seeds.split(": ")
        .collect::<Vec<&str>>()[1]
        .split(" ")
        .map(|ele| ele.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
        .chunks(2)
        .map(|chunk| {
            (chunk[0], chunk[0] + chunk[1])
        }).collect();
    seeds.sort_by_key(|pair| pair.0);
    let mut merged_seeds = Vec::new();
    for seed in &mut seeds {
        if merged_seeds.is_empty() {
            merged_seeds.push(seed);
        } else if seed.0 <= merged_seeds.last().unwrap().1 {
            merged_seeds.last_mut().unwrap().1 = merged_seeds.last().unwrap().1.max(seed.1);
        } else {
            merged_seeds.push(seed);
        }
    }
    return merged_seeds.iter().flat_map(|pair| pair.0..=pair.1).collect();
}

fn main() {
    let input = fs::read_to_string("test_input.txt")
        .expect("should read file");
    let content: Vec<&str> = input.split("\n\n").collect();
    let all_seeds = get_seeds(&content[0]);
    let maps: Vec<Vec<RangeMapping>> = content
        .iter()
        .skip(0)
        .map(|chunk| parse_map(chunk))
        .collect();
    let res = all_seeds
        .par_iter()
        .map(|seed| {
            let mut pipe_val: u64 = *seed;
            maps.iter().for_each(|map| {
                pipe_val = get_num(pipe_val, &map);
            });
            return pipe_val;
        }).min().unwrap();
    println!("{res}");

}

