use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::collections::HashMap;

const TARGET_NUM: i32 = 2020;

fn main() -> io::Result<()> {
    let file  = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut nums = Vec::new();

    for line in reader.lines() {
        nums.push(line?.parse::<i32>().unwrap());
    }

    match two_sum(&nums) {
        Some((a, b)) => println!("Two sum solution: {} * {} = {}", a, b, a*b),
        None         => println!("No two sum solution found."),
    }

    match three_sum(&nums) {
        Some((a, b, c)) => println!("Three sum solution: {} * {} * {} = {}", a, b, c, a*b*c),
        None            => println!("No three sum solution found."),
    }

    Ok(())
}

// Uses a lookup table to find the matching addend. O(n)
fn two_sum(nums: &[i32]) -> Option<(i32, i32)> {
    if nums.len() < 2 { return None; }
    let mut map = HashMap::new();
    nums.iter().for_each(|n| { map.insert(n, TARGET_NUM - n); });
    for (num, target) in &map {
        if map.contains_key(target) {
            return Some((**num, *target));
        }
    }
    None
}

fn three_sum(nums: &[i32]) -> Option<(i32, i32, i32)> {
    let length = nums.len();
    if length < 3 {
        return None;
    }

    let mut sorted = nums.to_vec();
    sorted.sort();

    for i in 0..length - 2 {
        let mut s = i+1;
        let mut e = sorted.len() - 1;
        let mut sum: i32;

        while s < e {
            sum = sorted[i] + sorted[s] + sorted[e];
            match sum {
                x if x == TARGET_NUM => return Some((sorted[i], sorted[s], sorted[e])),
                x if x < TARGET_NUM => s += 1,
                x if x > TARGET_NUM => e -= 1,
                _ => panic!("sum is not ==, >, or < than TARGET_NUM"),
            }
        }
    }
    None
}