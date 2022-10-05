#[aoc_generator(day4)]
fn input_generator(input: &str) -> (i32, i32) {
    let nums: Vec<i32> = input
        .split('-')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    (nums[0], nums[1])
}

#[aoc(day4, part1)]
fn p1(input: &(i32, i32)) -> usize {
    (input.0..=input.1)
        .into_iter()
        .filter(|p| password_check(*p))
        .count()
}

fn password_check(password: i32) -> bool {
    let digits: Vec<char> = password.to_string().chars().collect();

    for window in digits.windows(2) {
        let a = window[0];
        let b = window[1];

        if a > b {
            return false;
        }
    }

    todo!()
}
