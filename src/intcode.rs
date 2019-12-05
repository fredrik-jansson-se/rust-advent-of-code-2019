use super::helper::*;
use nom::bytes::complete::tag;
use nom::multi::separated_nonempty_list;
use nom::IResult;

pub fn parse_input(i: &str) -> IResult<&str, Vec<i32>> {
    separated_nonempty_list(tag(","), i32_val)(i)
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn from_int(i: i32) -> Self {
        match i {
            0 => Self::Position,
            1 => Self::Immediate,
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
    End,
}

impl Op {
    fn from_int(i: i32) -> Self {
        match i {
            1 => Self::Add,
            2 => Self::Mul,
            3 => Self::Store,
            4 => Self::Load,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LT,
            8 => Self::Eq,
            99 => Self::End,
            x => {
                dbg! {x};
                unreachable!()
            }
        }
    }
}

fn parse_op_code(code: i32) -> (Op, ParameterMode, ParameterMode, ParameterMode) {
    let op = Op::from_int(code % 100);
    let m1 = ParameterMode::from_int((code / 100) % 10);
    let m2 = ParameterMode::from_int((code / 1000) % 10);
    let m3 = ParameterMode::from_int((code / 10000) % 10);
    (op, m1, m2, m3)
}

fn get_value(data: &[i32], mode: ParameterMode, idx: usize) -> i32 {
    match mode {
        ParameterMode::Position => {
            let pos = data[idx] as usize;
            data[pos]
        }
        ParameterMode::Immediate => data[idx],
    }
}

pub fn run_program(data: &mut [i32], input: i32, return_idx: usize) -> (i32, Vec<i32>) {
    let mut pc = 0;
    let mut output = Vec::new();
    loop {
        let (op, m1, m2, _m3) = parse_op_code(data[pc]);
        match op {
            Op::Add => {
                let a = get_value(data, m1, pc + 1);
                let b = get_value(data, m2, pc + 2);
                let pd = data[pc + 3] as usize;
                data[pd] = a + b;
                pc += 4;
            }
            Op::Mul => {
                let a = get_value(data, m1, pc + 1);
                let b = get_value(data, m2, pc + 2);
                let pd = data[pc + 3] as usize;
                data[pd] = a * b;
                pc += 4;
            }
            Op::Load => {
                let v = get_value(data, m1, pc + 1);
                output.push(v);
                pc += 2;
            }
            Op::Store => {
                let pd = data[pc + 1] as usize;
                data[pd] = input;
                pc += 2;
            }
            Op::JumpIfTrue => {
                let v = get_value(data, m1, pc + 1);
                if v != 0 {
                    pc = get_value(data, m2, pc + 2) as usize;
                } else {
                    pc += 3;
                }
            }
            Op::JumpIfFalse => {
                let v = get_value(data, m1, pc + 1);
                if v == 0 {
                    pc = get_value(data, m2, pc + 2) as usize;
                } else {
                    pc += 3;
                }
            }
            Op::LT => {
                let a = get_value(data, m1, pc + 1);
                let b = get_value(data, m2, pc + 2);
                let pd = data[pc + 3] as usize;
                data[pd] = if a < b { 1 } else { 0 };
                pc += 4;
            }
            Op::Eq => {
                let a = get_value(data, m1, pc + 1);
                let b = get_value(data, m2, pc + 2);
                let pd = data[pc + 3] as usize;
                data[pd] = if a == b { 1 } else { 0 };
                pc += 4;
            }
            Op::End => break,
        }
    }
    (data[return_idx], output)
}

#[cfg(test)]
mod tests {

    #[test]
    fn intcode_parse() {
        use super::*;
        let input = "1,0,0,0,99";
        let (_, input) = parse_input(input).unwrap();
        assert_eq!(5, input.len());

        assert_eq!(parse_input("-1,2,4,-6"), Ok(("", vec![-1, 2, 4, -6])));
        assert_eq!(parse_input("-1,2,4,-6\n"), Ok(("\n", vec![-1, 2, 4, -6])));
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
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(2, run_program(&mut input, 0, 0).0);

        let input = "2,3,0,3,99";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(6, run_program(&mut input, 0, 3).0);

        let input = "2,4,4,5,99,0";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(9801, run_program(&mut input, 0, 5).0);

        let input = "1,1,1,4,99,5,6,0,99";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(30, run_program(&mut input, 0, 0).0);

        let input = "1002,4,3,4,33";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(99, run_program(&mut input, 0, 4).0);

        // Using position mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,9,8,9,10,9,4,9,99,-1,8";
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 0, 0);
        assert_eq!(0, output[output.len() - 1]);

        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 8, 0);
        assert_eq!(1, output[output.len() - 1]);

        // Using position mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,9,7,9,10,9,4,9,99,-1,8";
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 0, 0);
        assert_eq!(1, output[output.len() - 1]);

        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 8, 0);
        assert_eq!(0, output[output.len() - 1]);

        // Using immediate mode, consider whether the input is equal to 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,3,1108,-1,8,3,4,3,99";
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 0, 0);
        assert_eq!(0, output[output.len() - 1]);

        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 8, 0);
        assert_eq!(1, output[output.len() - 1]);

        // Using immediate mode, consider whether the input is less than 8; output 1 (if it is) or 0 (if it is not).
        let input = "3,3,1107,-1,8,3,4,3,99";
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 0, 0);
        assert_eq!(1, output[output.len() - 1]);

        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 8, 0);
        assert_eq!(0, output[output.len() - 1]);

        //  The program will then output 999 if the input value is below 8,
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 0, 0);
        assert_eq!(999, output[output.len() - 1]);

        //  output 1000 if the input value is equal to 8,
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 8, 0);
        assert_eq!(1000, output[output.len() - 1]);

        //  or output 1001 if the input value is greater than 8.
        let (_, mut code) = parse_input(input).unwrap();
        let (_, output) = run_program(&mut code, 88, 0);
        assert_eq!(1001, output[output.len() - 1]);
    }
}
