use super::helper::*;
use nom::bytes::complete::tag;
use nom::multi::separated_nonempty_list;
use nom::*;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day2.txt").unwrap();
    println!("2:1: {}", run_1(&input));
    println!("2:2: {}", run_2(&input));
}

fn parse_input(i: &str) -> IResult<&str, Vec<usize>> {
    separated_nonempty_list(tag(","), usize_val)(i)
}

fn run_1(input: &str) -> usize {
    let (_, mut data) = parse_input(input).unwrap();
    data[1] = 12;
    data[2] = 2;
    run_program(&mut data, 0)
}

fn run_2(input: &str) -> usize {
    let (_, orig_data) = parse_input(input).unwrap();
    for noun in 0..100 {
        for verb in 0..100 {
            let mut data = orig_data.clone();

            data[1] = noun;
            data[2] = verb;
            if 19690720 == run_program(&mut data, 0) {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

fn run_program(data: &mut [usize], return_idx: usize) -> usize {
    let mut pc = 0;
    loop {
        match data[pc] {
            1 => {
                let pa = data[pc + 1];
                let pb = data[pc + 2];
                let pd = data[pc + 3];
                data[pd] = data[pa] + data[pb];
                pc += 4;
            }
            2 => {
                let pa = data[pc + 1];
                let pb = data[pc + 2];
                let pd = data[pc + 3];
                data[pd] = data[pa] * data[pb];
                pc += 4;
            }
            99 => break,
            _ => unreachable!(),
        }
    }
    data[return_idx]
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc2_parse() {
        use super::*;
        let input = "1,0,0,0,99";
        let (_, input) = parse_input(input).unwrap();
        assert_eq!(5, input.len());
    }

    #[test]
    fn aoc2_run_program() {
        use super::*;
        let input = "1,0,0,0,99";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(2, run_program(&mut input, 0));

        let input = "2,3,0,3,99";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(6, run_program(&mut input, 3));

        let input = "2,4,4,5,99,0";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(9801, run_program(&mut input, 5));

        let input = "1,1,1,4,99,5,6,0,99";
        let (_, mut input) = parse_input(input).unwrap();
        assert_eq!(30, run_program(&mut input, 0));
    }
}
