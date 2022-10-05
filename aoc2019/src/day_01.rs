#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|l| {l.parse::<i32>().unwrap()}).collect()
}


#[aoc(day1, part1)]
fn p1(masses: &[i32]) -> i32 {
    masses.iter().map(calculate_mass_fuel).sum()
}

#[aoc(day1, part2)]
fn p2(masses: &[i32]) -> i32 {
    masses.iter().map(calculate_module_fuel).sum()
}

fn calculate_module_fuel(module: &i32) -> i32 {
    let mut total: i32 = 0;
    let mut fuel: i32 = calculate_mass_fuel(module);
    while fuel > 0 {
        total += fuel;
        fuel = calculate_mass_fuel(&fuel);
    }

    total
}

fn calculate_mass_fuel(mass: &i32) -> i32 {
    (mass / 3) - 2
}