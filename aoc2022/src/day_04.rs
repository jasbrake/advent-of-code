use std::ops::RangeInclusive;

use nom::{
    character::complete::{char, digit1, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

fn number(i: &str) -> IResult<&str, u32> {
    map_res(digit1, str::parse)(i)
}
fn range(i: &str) -> IResult<&str, RangeInclusive<u32>> {
    separated_pair(number, char('-'), number)(i).map(|(next, res)| (next, res.0..=res.1))
}

fn pair(i: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    separated_pair(range, char(','), range)(i)
}

fn all_pairs(i: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    separated_list1(newline, pair)(i)
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    all_pairs(input).expect("input failed to parse correctly").1
}

#[aoc(day4, part1)]
fn p1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(r1, r2)| {
            r1.contains(r2.start()) && r1.contains(r2.end())
                || r2.contains(r1.start()) && r2.contains(r1.end())
        })
        .count()
}

#[aoc(day4, part2)]
fn p2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> usize {
    input
        .iter()
        .filter(|(r1, r2)| {
            r1.contains(r2.start())
                || r1.contains(r2.end())
                || r2.contains(r1.start())
                || r2.contains(r1.end())
        })
        .count()
}
