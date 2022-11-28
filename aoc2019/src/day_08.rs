#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

const IMAGE_COLS: usize = 25;
const IMAGE_ROWS: usize = 6;
const IMAGE_SIZE: usize = IMAGE_ROWS * IMAGE_COLS;

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
        .min_by_key(|counts| counts.0)
        .unwrap();

    layer_counts_least_zeros.1 * layer_counts_least_zeros.2
}

struct Image(Vec<u32>);

impl std::fmt::Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut image_output = String::new();

        for (i, n) in self.0.iter().enumerate() {
            if i % IMAGE_COLS == 0 {
                image_output.push('\n');
            }

            image_output.push(match n {
                1 => 'X',
                _ => ' ',
            });

            image_output.push(' ');
        }

        write!(f, "{image_output}")
    }
}

#[aoc(day8, part2)]
fn p2(input: &[u32]) -> Image {
    let layers: Vec<&[u32]> = input.chunks(IMAGE_SIZE).collect();

    // Fold all the layers into one image, starting with a completely transparent image.
    Image(layers.iter().fold(vec![2; IMAGE_SIZE], |mut image, layer| {
        for (i, val) in layer.iter().enumerate() {
            if image[i] == 2 {
                image[i] = *val;
            }
        }

        image
    }))
}
