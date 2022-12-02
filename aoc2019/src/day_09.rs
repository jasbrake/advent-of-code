use std::sync::mpsc::channel;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .split(',')
        .map(|n| n.parse::<i64>().unwrap())
        .collect()
}

enum ParamMode {
    Position,
    Immediate,
    Relative(usize),
}

impl ParamMode {
    fn parse(n: i64, relative_base: usize) -> Self {
        match n {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative(relative_base),
            _ => panic!("unknown parameter mode"),
        }
    }

    fn value(self, mem: &[i64], param: i64) -> i64 {
        match self {
            ParamMode::Position => mem[param as usize],
            ParamMode::Immediate => param,
            ParamMode::Relative(base) => {
                let index = if param.is_negative() {
                    base.checked_sub(param.unsigned_abs() as usize)
                } else {
                    base.checked_add(param as usize)
                }
                .unwrap();

                mem[index]
            }
        }
    }
}

struct IntcodeComputer {
    mem: Vec<i64>,        // program memory, including intcode instructions
    ip: usize,            // instruction pointer
    input: Receiver<i64>, // input channel
    output: Sender<i64>,  // output channel
}

impl IntcodeComputer {
    pub fn new(program: &[i64], input: Receiver<i64>, output: Sender<i64>) -> Self {
        // Allocate 2^16 (65536) bytes of memory.
        let mut mem = vec![0; 0x10000];
        mem[..program.len()].copy_from_slice(program);

        IntcodeComputer {
            mem,
            ip: 0,
            input,
            output,
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        let mem = &mut self.mem;
        let mut relative_base: usize = 0;
        let mut last_output: Option<i64> = None;

        loop {
            let ins: i64 = mem[self.ip]; // undecoded instruction
            let op: i64 = ins % 100; // opcode

            // parameter modes
            let m1 = ParamMode::parse((ins % 1000) / 100, relative_base);
            let m2 = ParamMode::parse((ins % 10000) / 1000, relative_base);
            // Parameter mode is not necessary for parameter 3 as it is always used as a destination
            // address which will always be in Position mode.

            match op {
                // addition
                1 => {
                    let (p1, p2, p3) = (mem[self.ip + 1], mem[self.ip + 2], mem[self.ip + 3]);
                    mem[p3 as usize] = m1.value(mem, p1) + m2.value(mem, p2);
                    self.ip += 4;
                }

                // multiplication
                2 => {
                    let (p1, p2, p3) = (mem[self.ip + 1], mem[self.ip + 2], mem[self.ip + 3]);
                    mem[p3 as usize] = m1.value(mem, p1) * m2.value(mem, p2);
                    self.ip += 4;
                }

                // input
                3 => {
                    let p1 = mem[self.ip + 1];
                    mem[p1 as usize] = match self.input.recv() {
                        Ok(n) => n,
                        Err(_) => panic!("could not receive input"),
                    };
                    self.ip += 2;
                }

                // output
                4 => {
                    let p1 = mem[self.ip + 1];
                    let value = m1.value(mem, p1);
                    last_output = Some(value);
                    if self.output.send(value).is_err() {
                        // Sending on a closed channel, I guess terminate execution.
                        break;
                    }
                    self.ip += 2;
                }

                // jump-if-true
                5 => {
                    let (p1, p2) = (mem[self.ip + 1], mem[self.ip + 2]);
                    self.ip = if m1.value(mem, p1) != 0 {
                        m2.value(mem, p2) as usize
                    } else {
                        self.ip + 3
                    };
                }

                // jump-if-false
                6 => {
                    let (p1, p2) = (mem[self.ip + 1], mem[self.ip + 2]);
                    self.ip = if m1.value(mem, p1) == 0 {
                        m2.value(mem, p2) as usize
                    } else {
                        self.ip + 3
                    };
                }

                // less than
                7 => {
                    let (p1, p2, p3) = (mem[self.ip + 1], mem[self.ip + 2], mem[self.ip + 3]);
                    mem[p3 as usize] = i64::from(m1.value(mem, p1) < m2.value(mem, p2));
                    self.ip += 4
                }

                // equals
                8 => {
                    let (p1, p2, p3) = (mem[self.ip + 1], mem[self.ip + 2], mem[self.ip + 3]);
                    mem[p3 as usize] = i64::from(m1.value(mem, p1) == m2.value(mem, p2));
                    self.ip += 4
                }

                // adjust relative base
                9 => {
                    let p1 = mem[self.ip + 1];
                    relative_base = m1.value(mem, p1) as usize;
                    self.ip += 2;
                }

                // exit
                99 => break,
                _ => panic!("invalid opcode {}", op),
            }
        }

        // The program output is the last output before the program exits.
        last_output
    }
}

#[aoc(day9, part1)]
fn p1(input: &[i64]) -> Option<i64> {
    // TODO: Fix solution
    let (in_tx, in_rx) = channel::<i64>();
    let (out_tx, out_rx) = channel::<i64>();

    // Send start input
    in_tx.send(1).unwrap();

    let mut computer = IntcodeComputer::new(input, in_rx, out_tx);
    let ret = computer.run();

    for n in out_rx.iter() {
        println!("> {n}");
    }

    ret
}
