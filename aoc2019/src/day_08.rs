#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

const IMAGE_SIZE: usize = 25 * 6;

#[aoc(day8, part1)]
fn p1(input: &[u32]) -> u32 {
    let layers: Vec<&[u32]> = input.chunks(IMAGE_SIZE).collect();

    let layer_counts_least_zeros = layers
        .iter()
        .map(|l| {
            l.iter().fold((0, 0, 0), |(zeros, ones, twos), n| match n {
                0 => (zeros + 1, ones, twos),
                1 => (zeros, ones + 1, twos),
                2 => (zeros, ones, twos + 1),
                _ => (zeros, ones, twos),
            })
        })
        .min_by_key(|counts| counts.0).unwrap();

    layer_counts_least_zeros.1 * layer_counts_least_zeros.2
}
