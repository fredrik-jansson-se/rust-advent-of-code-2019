use std::fs;

pub fn run() {
    let input = fs::read_to_string("day5.txt").unwrap();

    println!("5:1 {}", run_1(&input));
    println!("5:2 {}", run_2(&input));
}

fn run_1(input: &str) -> i32 {
    let (_, mut data) = super::intcode::parse_input(input).unwrap();
    let (_, output) = super::intcode::run_program(&mut data, 1, 0);
    output[output.len() - 1]
}

fn run_2(input: &str) -> i32 {
    let (_, mut data) = super::intcode::parse_input(input).unwrap();
    let (_, output) = super::intcode::run_program(&mut data, 5, 0);
    output[output.len() - 1]
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc5_still_correct() {
        use super::*;
        let input = std::fs::read_to_string("day5.txt").unwrap();
        assert_eq!(run_1(&input), 9938601);
        assert_eq!(run_2(&input), 4283952);
    }
}
