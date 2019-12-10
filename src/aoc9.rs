use std::fs;

pub fn run() {
    let input = fs::read_to_string("day9.txt").unwrap();
    println!("day9-1: {}", run_1(&input));
    println!("day9-2: {}", run_2(&input));
}

fn run_1(program: &str) -> i64 {
    let mut cpu = crate::intcode::CPU::new(program);
    cpu.run(&mut vec![1]);
    cpu.output[cpu.output.len() - 1]
}

fn run_2(program: &str) -> i64 {
    let mut cpu = crate::intcode::CPU::new(program);
    cpu.run(&mut vec![2]);
    dbg! {&cpu.output};
    cpu.output[cpu.output.len() - 1]
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc9_run_1() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let mut cpu = crate::intcode::CPU::new(input);
        cpu.run(&mut vec![]);
        assert_eq!(
            cpu.output,
            [109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );

        let input = "1102,34915192,34915192,7,4,7,99,0";
        let mut cpu = crate::intcode::CPU::new(input);
        cpu.run(&mut vec![]);
        assert_eq!(cpu.output[0], 1219070632396864);

        let input = "104,1125899906842624,99";
        let mut cpu = crate::intcode::CPU::new(input);
        cpu.run(&mut vec![]);
        assert_eq!(cpu.output[0], 1125899906842624);
    }

    #[test]
    fn aoc9_still_correct() {
        use super::*;
        let input = fs::read_to_string("day9.txt").unwrap();
        assert_eq!(2932210790, run_1(&input));
        assert_eq!(73144, run_2(&input));
    }
}
