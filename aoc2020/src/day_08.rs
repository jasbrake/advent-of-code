use std::error;
use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone)]
enum ParseOpcodeError {
    InvalidOp,
    Parse(ParseIntError),
}

impl fmt::Display for ParseOpcodeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseOpcodeError::InvalidOp => write!(f, "unknown op code"),
            ParseOpcodeError::Parse(..) => {
                write!(f, "the provided string could not be parse as int")
            }
        }
    }
}

impl error::Error for ParseOpcodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseOpcodeError::InvalidOp => None,
            ParseOpcodeError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ParseOpcodeError {
    fn from(err: ParseIntError) -> ParseOpcodeError {
        ParseOpcodeError::Parse(err)
    }
}

#[derive(Debug, Clone)]
enum Opcode {
    Acc(i32),
    Jmp(i32),
    Nop,
}

impl FromStr for Opcode {
    type Err = ParseOpcodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split_whitespace();
        let op = split.next();
        let value = split.next().unwrap().parse::<i32>()?;

        match (op, value) {
            (Some("acc"), v) => Ok(Opcode::Acc(v)),
            (Some("jmp"), v) => Ok(Opcode::Jmp(v)),
            (Some("nop"), _) => Ok(Opcode::Nop),
            (_, _) => Err(ParseOpcodeError::InvalidOp),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    op: Opcode,
    executed: bool,
}

impl Instruction {
    fn new(op: Opcode) -> Self {
        Self {
            op,
            executed: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Program {
    instructions: Vec<Instruction>,
    pc: usize,
    val: i32,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Self {
        Self {
            instructions,
            pc: 0,
            val: 0,
        }
    }

    fn execute_pc(&mut self) {
        let ins = &mut self.instructions[self.pc];
        ins.executed = true;

        match ins.op {
            Opcode::Acc(v) => {
                self.val += v;
                self.pc += 1;
            }

            Opcode::Jmp(v) => {
                if v < 0 {
                    if v.abs() as usize > self.pc {
                        panic!("Error executing program: PC underflowed")
                    }
                    self.pc -= v.abs() as usize;
                } else {
                    self.pc += v as usize;
                }
            }

            Opcode::Nop => self.pc += 1,
        }
    }

    fn run(&mut self) {
        loop {
            if self.pc > self.instructions.len() {
                panic!("PC higher than number of instructions")
            }

            let Instruction { executed, .. } = self.instructions[self.pc];
            if executed {
                break;
            }
            self.execute_pc();
        }
    }

    fn run_and_fix(&mut self) {
        loop {
            // TODO detect when program correctly finishes
            self.run()
        }
    }
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Opcode::from_str)
        .map(Result::unwrap)
        .map(Instruction::new)
        .collect()
}

#[aoc(day8, part1)]
fn acc_value_first_run(instructions: &[Instruction]) -> i32 {
    let mut program = Program::new(instructions.to_vec());
    program.run();
    program.val
}

#[aoc(day8, part2)]
fn program_fix_loop(instructions: &[Instruction]) -> i32 {
    let mut program = Program::new(instructions.to_vec());
    program.run();
    program.val
}
