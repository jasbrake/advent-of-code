use std::collections::HashMap;

const TARGET_NUM: i32 = 2020;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i32> {
    input
      .lines()
      .map(|l| l.parse::<i32>().unwrap())
      .collect()
}

// Uses a lookup table to find the matching addend. O(n)
#[aoc(day1, part1)]
fn two_sum(nums: &[i32]) -> Option<i32> {
  if nums.len() < 2 {
    return None;
  }
    let mut map = HashMap::new();
    nums.iter().for_each(|n| { map.insert(n, TARGET_NUM - n); });
    for (num, target) in &map {
        if map.contains_key(target) {
            return Some(*num * target);
        }
    }
    None
}

#[aoc(day1, part2)]
fn three_sum(nums: &[i32]) -> Option<i32> {
    let length = nums.len();
    if length < 3 {
        return None;
    }

    let mut sorted = nums.to_vec();
    sorted.sort_unstable();

    for i in 0..length - 2 {
        let mut s = i+1;
        let mut e = sorted.len() - 1;
        let mut sum: i32;

        while s < e {
            sum = sorted[i] + sorted[s] + sorted[e];
            match sum {
                x if x == TARGET_NUM => return Some(sorted[i] * sorted[s] * sorted[e]),
                x if x < TARGET_NUM => s += 1,
                x if x > TARGET_NUM => e -= 1,
                _ => panic!("sum is not ==, >, or < than TARGET_NUM"),
            }
        }
    }
    None
}