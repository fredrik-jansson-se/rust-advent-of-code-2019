use super::helper::*;
use nom::{bytes::complete::tag, multi::separated_list1, IResult};

fn parse_program(i: &str) -> IResult<&str, Vec<i64>> {
    separated_list1(tag(","), i64_val)(i)
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_int(i: i64) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
            2 => Self::Relative,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum Op {
    Add,
    Mul,
    Store,
    Load,
    JumpIfTrue,
    JumpIfFalse,
    LT,
    Eq,
    AdjRelBase,
    End,
}

impl Op {
    fn from_int(i: i64) -> Self {
        match i {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Store,
            4 => Self::Load,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LT,
            8 => Self::Eq,
            9 => Self::AdjRelBase,
            99 => Self::End,
            x => {
                dbg! {x};
                unreachable!()
            }
        }
    }
}

fn parse_op_code(code: i64) -> (Op, ParameterMode, ParameterMode, ParameterMode) {
    let op = Op::from_int(code % 100);
    let m1 = ParameterMode::from_int((code / 100) % 10);
    let m2 = ParameterMode::from_int((code / 1000) % 10);
    let m3 = ParameterMode::from_int((code / 10000) % 10);
    (op, m1, m2, m3)
}

#[derive(Debug, PartialEq)]
pub enum State {
    Running,
    Exited,
    NeedInput,
}

#[derive(Clone)]
pub struct CPU {
    pc: usize,
    relative_base: i64,
    pub memory: Vec<i64>,
    pub output: Vec<i64>,
}

impl CPU {
    pub fn new(program: &str) -> Self {
        let (_, mem) = parse_program(program).unwrap();
        CPU {
            pc: 0,
            relative_base: 0,
            memory: mem,
            output: Vec::new(),
        }
    }

    fn get_value(&self, mode: ParameterMode, idx: usize) -> i64 {
        let read_pos = match mode {
            ParameterMode::Position => self.memory[idx] as usize,
            ParameterMode::Relative => {
                let rel = self.memory[idx] as i64;
                (self.relative_base + rel) as usize
            }
            ParameterMode::Immediate => idx,
        };

        if read_pos < self.memory.len() {
            self.memory[read_pos]
        } else {
            0
        }
    }

    fn set_value(&mut self, mode: ParameterMode, idx: usize, val: i64) {
        let write_pos = match mode {
            ParameterMode::Position => self.memory[idx] as usize,
            ParameterMode::Relative => {
                let rel = self.memory[idx] as i64;
                (self.relative_base + rel) as usize
            }
            ParameterMode::Immediate => idx,
        };

        if write_pos >= self.memory.len() {
            self.memory.resize(write_pos * 2, 0);
        }
        self.memory[write_pos] = val;
    }

    fn step(&mut self, input: &mut Vec<i64>) -> State {
        let (op, m1, m2, m3) = parse_op_code(self.memory[self.pc]);
        match op {
            Op::Add => {
                let a = self.get_value(m1, self.pc + 1);
                let b = self.get_value(m2, self.pc + 2);
                self.set_value(m3, self.pc + 3, a + b);
                self.pc += 4;
            }
            Op::Mul => {
                let a = self.get_value(m1, self.pc + 1);
                let b = self.get_value(m2, self.pc + 2);
                self.set_value(m3, self.pc + 3, a * b);
                self.pc += 4;
            }
            Op::Load => {
                let v = self.get_value(m1, self.pc + 1);
                self.output.push(v);
                self.pc += 2;
            }
            Op::Store => {
                if input.is_empty() {
                    return State::NeedInput;
                } else {
                    self.set_value(m1, self.pc + 1, input.remove(0));
                    self.pc += 2;
                }
            }
            Op::JumpIfTrue => {
                let v = self.get_value(m1, self.pc + 1);
                if v != 0 {
                    self.pc = self.get_value(m2, self.pc + 2) as usize;
                } else {
                    self.pc += 3;
                }
            }
            Op::JumpIfFalse => {
                let v = self.get_value(m1, self.pc + 1);
                if v == 0 {
                    self.pc = self.get_value(m2, self.pc + 2) as usize;
                } else {
                    self.pc += 3;
                }
            }
            Op::LT => {
                let a = self.get_value(m1, self.pc + 1);
                let b = self.get_value(m2, self.pc + 2);
                self.set_value(m3, self.pc + 3, if a < b { 1 } else { 0 });
                self.pc += 4;
            }
            Op::AdjRelBase => {
                self.relative_base += self.get_value(m1, self.pc + 1);
                self.pc += 2;
            }
            Op::Eq => {
                let a = self.get_value(m1, self.pc + 1);
                let b = self.get_value(m2, self.pc + 2);
                self.set_value(m3, self.pc + 3, if a == b { 1 } else { 0 });
                self.pc += 4;
            }
            Op::End => return State::Exited,
        }
        State::Running
    }

    pub fn run(&mut self, input: &mut Vec<i64>) -> State {
        loop {
            let st = self.step(input);
            if st != State::Running {
                return st;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn intcode_parse() {
        use super::*;
        let input = "1,0,0,0,99";
        let (_, input) = parse_program(input).unwrap();
        assert_eq!(5, input.len());

        assert_eq!(parse_program("-1,2,4,-6"), Ok(("", vec![-1, 2, 4, -6])));
        assert_eq!(parse_program("-1,2,4,-6\n"), Ok(("\n", vec![-1, 2, 4, -6])));
    }

    #[test]
    fn intcode_ops() {
        use super::*;
        assert_eq!(
            parse_op_code(1002),
            (
                Op::Mul,
                ParameterMode::Position,
                ParameterMode::Immediate,
                ParameterMode::Position
            )
        );
    }

    #[test]
    fn intcode_run_program() {
        use super::*;
        let input = "1,0,0,0,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(2, cpu.memory[0]);

        let input = "2,3,0,3,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(6, cpu.memory[3]);

        let input = "2,4,4,5,99,0";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(9801, cpu.memory[5]);

        let input = "1,1,1,4,99,5,6,0,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(30, cpu.memory[0]);

        let input = "1002,4,3,4,33";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(99, cpu.memory[4]);

        // // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(0, cpu.output[cpu.output.len() - 1]);

        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![8]);
        assert_eq!(1, cpu.output[cpu.output.len() - 1]);

        // // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(1, cpu.output[cpu.output.len() - 1]);

        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![8]);
        assert_eq!(0, cpu.output[cpu.output.len() - 1]);

        // // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,3,1108,-1,8,3,4,3,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(0, cpu.output[cpu.output.len() - 1]);

        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![8]);
        assert_eq!(1, cpu.output[cpu.output.len() - 1]);

        // // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,3,1107,-1,8,3,4,3,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(1, cpu.output[cpu.output.len() - 1]);

        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![8]);
        assert_eq!(0, cpu.output[cpu.output.len() - 1]);

        // //  The program will then output 999 if the input value is below 8,
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![0]);
        assert_eq!(999, cpu.output[cpu.output.len() - 1]);

        // //  output 1000 if the input value is equal to 8,
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![8]);
        assert_eq!(1000, cpu.output[cpu.output.len() - 1]);

        // //  or output 1001 if the input value is greater than 8.
        let mut cpu = CPU::new(input);
        cpu.run(&mut vec![88]);
        assert_eq!(1001, cpu.output[cpu.output.len() - 1]);

        // Test relative store
        let mut cpu = CPU::new("203,10,99");
        cpu.run(&mut vec![88]);
        assert_eq!(88, cpu.memory[10]);

        let mut cpu = CPU::new("109,10,203,10,99");
        cpu.run(&mut vec![88]);
        assert_eq!(88, cpu.memory[20]);

        let mut cpu = CPU::new("109,10,203,-10,99");
        cpu.run(&mut vec![88]);
        assert_eq!(88, cpu.memory[0]);
    }
}
