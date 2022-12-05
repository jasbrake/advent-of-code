use std::collections::HashSet;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .into_iter()
        .map(|l| l.chars().collect())
        .collect()
}

fn priority(c: &char) -> u32 {
    if c.is_uppercase() {
        (*c as u32 - 0x40) + 26
    } else {
        *c as u32 - 0x60
    }
}

#[aoc(day3, part1)]
fn p1(input: &[Vec<char>]) -> u32 {
    input
        .iter()
        .flat_map(|bag| {
            let middle = bag.len() / 2;
            let front = bag[..middle].iter().copied().collect::<HashSet<char>>();
            let back = bag[middle..].iter().copied().collect::<HashSet<char>>();

            front.intersection(&back).copied().collect::<Vec<char>>()
        })
        .collect::<Vec<char>>()
        .iter()
        .map(priority)
        .sum::<u32>()
}

#[aoc(day3, part2)]
fn p2(input: &[Vec<char>]) -> u32 {
    input
        .chunks_exact(3)
        .flat_map(|bags| {
            let set1 = bags[0].iter().copied().collect::<HashSet<char>>();
            let set2 = bags[1].iter().copied().collect::<HashSet<char>>();
            let set3 = bags[2].iter().copied().collect::<HashSet<char>>();

            set1.intersection(&set2)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&set3)
                .copied()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<char>>()
        .iter()
        .map(priority)
        .sum::<u32>()
}
