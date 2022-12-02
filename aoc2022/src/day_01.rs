use std::collections::BinaryHeap;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .collect::<Vec<&str>>()
        .split(|l| l.is_empty())
        .map(|array| array.iter().map(|l| l.parse::<i32>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
fn p1(input: &[Vec<i32>]) -> Option<i32> {
    input.iter().map(|calories| calories.iter().sum()).max()
}

#[aoc(day1, part2)]
fn p2(input: &[Vec<i32>]) -> i32 {
    input
        .iter()
        .map(|calories| calories.iter().sum())
        .collect::<BinaryHeap<i32>>()
        .into_iter_sorted()
        .take(3)
        .sum()
}
