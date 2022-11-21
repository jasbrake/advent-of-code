#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Password> {
    let nums: Vec<i32> = input
        .split('-')
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    (nums[0]..=nums[1]).map(Password::new).collect()
}

struct Password {
    _value: i32,
    chars: Vec<char>,
}

impl Password {
    fn new(value: i32) -> Self {
        Self {
            _value: value,
            chars: value.to_string().chars().collect(),
        }
    }
}

enum Rule {
    Length(usize),
    Consecutive,
    StrictConsecutive,
    IncreasingChars,
}

struct PasswordValidator {
    rules: Vec<Rule>,
}

impl PasswordValidator {
    fn validate(&self, password: &Password) -> bool {
        for rule in self.rules.iter() {
            match *rule {
                Rule::Length(n) => {
                    if password.chars.len() != n {
                        return false;
                    }
                }

                Rule::Consecutive => {
                    let mut consecutive = false;
                    let mut last_char = password.chars[0];

                    for c in password.chars.iter().skip(1) {
                        if *c == last_char {
                            consecutive = true;
                        } else {
                            last_char = *c;
                        }
                    }

                    if !consecutive {
                        return false;
                    }
                }

                Rule::StrictConsecutive => {
                    let mut count = 1;
                    let mut last_char = password.chars[0];

                    for c in password.chars.iter().skip(1) {
                        if *c == last_char {
                            count += 1;
                        } else if count == 2 {
                            break;
                        } else {
                            count = 1;
                            last_char = *c;
                        }
                    }

                    if count != 2 {
                        return false;
                    }
                }

                Rule::IncreasingChars => {
                    for window in password.chars.windows(2) {
                        let a = window[0];
                        let b = window[1];

                        if b < a {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[aoc(day4, part1)]
fn p1(input: &[Password]) -> usize {
    let validator = PasswordValidator {
        rules: vec![Rule::Length(6), Rule::Consecutive, Rule::IncreasingChars],
    };

    input.iter().filter(|p| validator.validate(p)).count()
}

#[aoc(day4, part2)]
fn p2(input: &[Password]) -> usize {
    let validator = PasswordValidator {
        rules: vec![Rule::Length(6), Rule::StrictConsecutive, Rule::IncreasingChars],
    };

    input.iter().filter(|p| validator.validate(p)).count()
}