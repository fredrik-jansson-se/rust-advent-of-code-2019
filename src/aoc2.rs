use std::fs;

pub fn run() {
    let input = fs::read_to_string("day2.txt").unwrap();
    println!("2:1: {}", run_1(&input));
    println!("2:2: {}", run_2(&input));
}

fn run_1(input: &str) -> i32 {
    let mut cpu = super::intcode::CPU::new(input);
    cpu.memory[1] = 12;
    cpu.memory[2] = 2;
    cpu.run(&mut vec![0]);
    // super::intcode::run_program(&mut data, 0, 0).0;
    cpu.memory[0]
}

fn run_2(input: &str) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut cpu = super::intcode::CPU::new(input);
            // let mut data = orig_data.clone();

            cpu.memory[1] = noun;
            cpu.memory[2] = verb;
            cpu.run(&mut vec![0]);
            if 19690720 == cpu.memory[0] {
                return 100 * noun + verb;
            }
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc2_still_correct() {
        use super::*;
        let input = std::fs::read_to_string("day2.txt").unwrap();
        assert_eq!(run_1(&input), 7594646);
        assert_eq!(run_2(&input), 3376);
    }
}
