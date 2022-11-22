#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

enum ParamMode {
    Position,
    Immediate,
}

impl ParamMode {
    fn parse(n: i32) -> Self {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => panic!("unknown parameter mode"),
        }
    }

    fn value(self, program: &[i32], param: i32) -> i32 {
        match self {
            ParamMode::Position => program[param as usize],
            ParamMode::Immediate => param,
        }
    }
}

fn run(program: &mut [i32], input: i32) -> Vec<i32> {
    let mut output = vec![];
    let mut pc: usize = 0; // instruction pointer

    loop {
        let ins: i32 = program[pc]; // undecoded instruction
        let op: i32 = ins % 100; // opcode

        // parameter modes
        let pm1 = ParamMode::parse((ins % 1000) / 100);
        let pm2 = ParamMode::parse((ins % 10000) / 1000);
        // Parameter mode is not necessary for parameter 3 as it is always used as a destination
        // address which will always be in Position mode.

        match op {
            // addition
            1 => {
                let (p1, p2, p3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[p3 as usize] = pm1.value(program, p1) + pm2.value(program, p2);
                pc += 4;
            }

            // multiplication
            2 => {
                let (p1, p2, p3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[p3 as usize] = pm1.value(program, p1) * pm2.value(program, p2);
                pc += 4;
            }

            // input
            3 => {
                let p1 = program[pc + 1];
                program[p1 as usize] = input;
                pc += 2;
            }

            // output
            4 => {
                let p1 = program[pc + 1];
                output.push(pm1.value(program, p1));
                pc += 2;
            }

            // jump-if-true
            5 => {
                let (p1, p2) = (program[pc + 1], program[pc + 2]);
                pc = if pm1.value(program, p1) != 0 {
                    pm2.value(program, p2) as usize
                } else {
                    pc + 3
                };
            }

            // jump-if-false
            6 => {
                let (p1, p2) = (program[pc + 1], program[pc + 2]);
                pc = if pm1.value(program, p1) == 0 {
                    pm2.value(program, p2) as usize
                } else {
                    pc + 3
                };
            }

            // less than
            7 => {
                let (p1, p2, p3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[p3 as usize] = i32::from(pm1.value(program, p1) < pm2.value(program, p2));
                pc += 4
            }

            // equals
            8 => {
                let (p1, p2, p3) = (program[pc + 1], program[pc + 2], program[pc + 3]);
                program[p3 as usize] = i32::from(pm1.value(program, p1) == pm2.value(program, p2));
                pc += 4
            }

            // exit
            99 => break,
            _ => panic!("invalid opcode"),
        }
    }

    output
}

#[aoc(day5, part1)]
fn p1(input: &[i32]) -> Option<i32> {
    let mut program = input.to_owned();
    run(&mut program, 1).last().copied()
}

#[aoc(day5, part2)]
fn p2(input: &[i32]) -> Option<i32> {
    let mut program = input.to_owned();
    run(&mut program, 5).last().copied()
}
