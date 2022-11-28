use itertools::Itertools;
use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[aoc_generator(day7)]
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

fn run(program: &mut [i32], input: Receiver<i32>, output: Sender<i32>) -> Option<i32> {
    // let mut output = vec![];
    // let mut input_iter = input.iter();
    let mut last_output: Option<i32> = None;
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
                program[p1 as usize] = match input.recv() {
                    Ok(n) => n,
                    Err(_) => panic!("could not receive input"),
                };
                pc += 2;
            }

            // output
            4 => {
                let p1 = program[pc + 1];
                let value = pm1.value(program, p1);
                last_output = Some(value);
                if output.send(value).is_err() {
                    // Sending on a closed channel, I guess terminate execution.
                    break;
                }
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

    // The program output is the last output before the program exits.
    last_output
}

#[aoc(day7, part1)]
fn p1(input: &[i32]) -> Option<i32> {
    (0..=4)
        .permutations(5)
        .map(|phase_settings| {
            let mut threads = vec![];

            let (mut previous_tx, mut previous_rx) = channel::<i32>();
            let first_tx = previous_tx.clone();

            // Create all amplifiers but the first, so that we can link the input/output channels
            // correctly.
            let mut phase_iter = phase_settings.iter().peekable();
            while let Some(phase) = phase_iter.next() {
                let mut program = input.to_owned();

                // Create the output channel for this amplifier.
                let (tx, rx) = channel::<i32>();

                // Send our phase setting into the send side of this amplifier's input channel.
                previous_tx.send(*phase).unwrap();
                previous_tx = tx.clone();

                let output = if phase_iter.peek().is_none() { first_tx.clone() } else { tx };

                threads.push(thread::spawn(move || {
                    run(&mut program, previous_rx, output)
                        .expect("each program should have at least one output")
                }));

                previous_rx = rx;
            }

            // Start the first amplifier by giving it the input signal.
            first_tx.send(0).unwrap();

            // Output value will be the last value output by the last amplifier.
            threads.into_iter().last().unwrap().join().unwrap()
        })
        .max()
}

#[aoc(day7, part2)]
fn p2(input: &[i32]) -> Option<i32> {
    (5..=9)
        .permutations(5)
        .map(|phase_settings| {
            let mut threads = vec![];

            let (mut previous_tx, mut previous_rx) = channel::<i32>();
            let first_tx = previous_tx.clone();

            // Create all amplifiers but the first, so that we can link the input/output channels
            // correctly.
            let mut phase_iter = phase_settings.iter().peekable();
            while let Some(phase) = phase_iter.next() {
                let mut program = input.to_owned();

                // Create the output channel for this amplifier.
                let (tx, rx) = channel::<i32>();

                // Send our phase setting into the send side of this amplifier's input channel.
                previous_tx.send(*phase).unwrap();
                previous_tx = tx.clone();

                let output = if phase_iter.peek().is_none() { first_tx.clone() } else { tx };

                threads.push(thread::spawn(move || {
                    run(&mut program, previous_rx, output)
                        .expect("each program should have at least one output")
                }));

                previous_rx = rx;
            }

            // Start the first amplifier by giving it the input signal.
            first_tx.send(0).unwrap();

            // Output value will be the last value output by the last amplifier.
            threads.into_iter().last().unwrap().join().unwrap()
        })
        .max()
}
