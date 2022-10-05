fn parse_seat(s: &str) -> u32 {
  let mut n: u32 = 0;
  for (i, c) in s.chars().rev().enumerate() {
    if c == 'B' || c == 'R' {
      n |= 1 << i;
    }
  }
  n
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<u32> {
  input
    .split('\n')
    .map(parse_seat)
    .collect()
}

#[aoc(day5, part1)]
fn max_seat_id(input: &[u32]) -> u32 {
  *input.iter().max().unwrap()
}

#[aoc(day5, part2)]
fn missing_seat(input: &[u32]) -> u32 {
  let mut min: u32 = std::u32::MAX;
  let mut max: u32 = std::u32::MIN;
  let mut sum: u32 = 0;
  input.iter().for_each(|n| {
    match *n {
      x if x > max => max = x,
      x if x < min => min = x,
      _ => {},
    }
    sum += *n;
  });
  let complete_sum: u32 = (min..=max)
    .collect::<Vec<u32>>()
    .iter()
    .sum();
  complete_sum - sum
}
