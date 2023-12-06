use std::fs;

fn num_ways(total_time: &u64, dist: &u64) -> u64{
    let res = (0..(*total_time)).fold(0, |acc, time_holding| {
        match &((total_time - time_holding) * time_holding) > dist {
            true  => acc + 1,
            false => acc
        }
    });
    res
}

fn main() {
    let nums: Vec<u64> = fs::read_to_string("input.txt")
        .expect("should read file")
        .lines()
        .map(|line| {
            let nums = line.split(":").collect::<Vec<&str>>()[1];
            return nums.trim().split_whitespace().collect::<String>().parse().unwrap();
        })
        .collect();

    let (times, dist) = (&nums[0], &nums[1]);
    let res = num_ways(&times, &dist);
    println!("{res}");
}

