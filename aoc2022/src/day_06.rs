use std::collections::HashSet;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn distinct_count_index(input: &[char], distinct_count: usize) -> Option<usize> {
    for (i, window) in input.windows(distinct_count).enumerate() {
        let unique: HashSet<char> = window.into_iter().cloned().collect();

        if unique.len() == distinct_count {
            return Some(i + distinct_count);
        }
    }

    None
}

#[aoc(day6, part1)]
fn p1(input: &[char]) -> Option<usize> {
    distinct_count_index(input, 4)
}

#[aoc(day6, part2)]
fn p2(input: &[char]) -> Option<usize> {
    distinct_count_index(input, 14)
}
