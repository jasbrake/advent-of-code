use regex::Regex;

#[derive(Debug)]
struct PasswordPolicy {
    password: String,
    policy_char: char,
    policy_min: usize,
    policy_max: usize,
}

impl PasswordPolicy {
    fn parse(s: &str) -> PasswordPolicy {
        let re = Regex::new(r"^(?P<lower>\d+)-(?P<upper>\d+)\s(?P<character>.):\s(?P<password>.+)").unwrap();
        let cap = re.captures(s).unwrap();
        PasswordPolicy {
            password: cap.name("password").unwrap().as_str().to_owned(),
            policy_char: cap.name("character").unwrap().as_str().parse::<char>().unwrap(),
            policy_min: cap.name("lower").unwrap().as_str().parse::<usize>().unwrap(),
            policy_max: cap.name("upper").unwrap().as_str().parse::<usize>().unwrap(),
        }
    }

    fn p1_valid(&self) -> bool {
        let char_count = self.password
            .chars()
            .filter(|c| { *c == self.policy_char })
            .count();
        char_count >= self.policy_min && char_count <= self.policy_max
    }

    fn p2_valid(&self) -> bool {
        (self.password.chars().nth(self.policy_min-1).unwrap() == self.policy_char) ^ (self.password.chars().nth(self.policy_max-1).unwrap() == self.policy_char)
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<PasswordPolicy> {
    input
      .lines()
      .map(PasswordPolicy::parse)
      .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[PasswordPolicy]) -> usize {
  input
    .iter()
    .filter(|p| p.p1_valid())
    .count()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[PasswordPolicy]) -> usize {
  input
    .iter()
    .filter(|p| p.p2_valid())
    .count()
}
