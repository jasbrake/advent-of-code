use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
struct Passport {
  byr: String,         // (Birth Year)
  iyr: String,         // (Issue Year)
  eyr: String,         // (Expiration Year)
  hgt: String,         // (Height)
  hcl: String,         // (Hair Color)
  ecl: String,         // (Eye Color)
  pid: String,         // (Passport ID)
  cid: Option<String>, // (Country ID)
}

impl Passport {
  fn is_valid(&self) -> bool {
    let color_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    self.byr.len() == 4 &&
    validate_range(&self.byr, 1920, 2002) &&

    self.iyr.len() == 4 &&
    validate_range(&self.iyr, 2010, 2020) &&

    self.eyr.len() == 4 &&
    validate_range(&self.eyr, 2020, 2030) &&

    match &self.hgt[self.hgt.len()-2..] {
      "cm" => validate_range(&self.hgt[..3], 150, 193) && self.hgt.len() == 5,
      "in" => validate_range(&self.hgt[..2], 59, 76) && self.hgt.len() == 4,
      _ => false,
    } &&

    self.hcl.len() == 7 &&
    color_re.is_match(&self.hcl) &&

    matches!(self.ecl.as_str(), "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") &&

    self.pid.len() == 9
  }
}

fn validate_range(s: &str, min: i32, max: i32) -> bool {
  let n = s.parse::<i32>();
  match n {
    Ok(n)  => n >= min && n <= max,
    Err(_) => false,
  }
}

fn parse_passport(s: &str) -> Option<Passport> {
  let mut map = HashMap::new();
  s.split_ascii_whitespace()
    .for_each(|item| {
      let split: Vec<&str> = item.split(':').collect();
      map.insert(split[0], split[1]);
    });

  Some(Passport {
    byr: map.get("byr")?.to_string(),
    iyr: map.get("iyr")?.to_string(),
    eyr: map.get("eyr")?.to_string(),
    hgt: map.get("hgt")?.to_string(),
    hcl: map.get("hcl")?.to_string(),
    ecl: map.get("ecl")?.to_string(),
    pid: map.get("pid")?.to_string(),
    cid: map.get("cid").map(|x| x.to_string()),
  })
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Passport> {
  let entries = input.split("\n\n");
  entries.filter_map(|e| parse_passport(e)).collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
  input.len()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
  input
    .iter()
    .filter(|p| p.is_valid())
    .count()
}
