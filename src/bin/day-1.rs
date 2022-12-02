use std::cmp::max;
use std::io;
use std::io::BufRead;

fn main() {
    let mut max_sum: u64 = 0;
    let mut curr_sum: u64 = 0;

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(value) = line.unwrap().parse::<u64>() {
            curr_sum += value;
        } else {
            max_sum = max(max_sum, curr_sum);
            curr_sum = 0;
        }
    }

    max_sum = max(max_sum, curr_sum);
    println!("{}", max_sum);
}
