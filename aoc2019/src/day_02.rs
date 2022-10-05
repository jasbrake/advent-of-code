use std::fmt;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<i32> {
    input.split(',').map(|n| {n.parse::<i32>().unwrap()}).collect()
}

#[aoc(day2, part1)]
fn p1(input: &[i32]) -> i32 {
    let mut program = input.to_owned();
    program[1] = 12;
    program[2] = 2;

    run(&mut program);

    program[0]
}

struct Pair(i32, i32);

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[aoc(day2, part2)]
fn p2(input: &[i32]) -> Option<Pair> {
    const ANSWER: i32 = 19690720;

    for i in 0..=99 {
        for j in 0..=99 {
            let mut program = input.to_owned();
            program[1] = i;
            program[2] = j;
            run(&mut program);

            match program[0].cmp(&ANSWER) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => return Some(Pair(i, j)),
                std::cmp::Ordering::Greater => break,
            }
        }
    }

    None
}

fn run(program: &mut [i32]) {
    let mut pc: usize = 0; // program counter

    loop {
        match program[pc] {
            1 => {
                let a = program[pc+1] as usize;
                let b = program[pc+2] as usize;
                let dest = program[pc+3] as usize;
                program[dest] = program[a] + program[b];
                pc += 4;
            },
            2 => {
                let a = program[pc+1] as usize;
                let b = program[pc+2] as usize;
                let dest = program[pc+3] as usize;
                program[dest] = program[a] * program[b];
                pc += 4;
            },
            99 => break,
            _ => panic!("invalid opcode"),
        }
    }
}