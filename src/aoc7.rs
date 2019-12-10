use permutohedron::Heap;
use rayon::prelude::*;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day7.txt").unwrap();
    println!("day7-1: {}", run_1(&input));
    println!("day7-2: {}", run_2(&input));
}

fn run_amp_1(program: &str, phases: &[i64]) -> i64 {
    let mut io = 0;

    for p in phases {
        let mut amp = super::intcode::CPU::new(program);
        let mut input = vec![*p, io];
        amp.run(&mut input);
        io = amp.output[0]
    }
    io
}

pub fn run_1(input: &str) -> i64 {
    let mut data = [0, 1, 2, 3, 4];
    let heap = Heap::new(&mut data);
    let phases = heap.collect::<Vec<_>>();

    phases
        .par_iter()
        .map(|p| run_amp_1(input, p))
        .max()
        .unwrap()
}

fn run_amp_2(program: &str, phases: &[i64]) -> i64 {
    let mut amps = (0..5)
        .map(|_| super::intcode::CPU::new(program))
        .collect::<Vec<_>>();
    let mut inputs: Vec<Vec<i64>> = phases.iter().map(|p| vec![*p]).collect::<Vec<_>>();
    // Initial input for amp A
    inputs[0].push(0);

    loop {
        for (i, amp) in amps.iter_mut().enumerate() {
            let ret = amp.run(&mut inputs[i]);
            let il = inputs.len();
            inputs[(i + 1) % il].push(amp.output[amp.output.len() - 1]);
            if i == 4 && ret == super::intcode::State::Exited {
                return amp.output[amp.output.len() - 1];
            }
        }
    }
}

pub fn run_2(input: &str) -> i64 {
    let mut data = [5, 6, 7, 8, 9];
    let heap = Heap::new(&mut data);
    let phases = heap.collect::<Vec<_>>();

    phases
        .par_iter()
        .map(|p| run_amp_2(input, p))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    //
    #[test]
    fn aoc7_1() {
        use super::*;
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        assert_eq!(43210, run_amp_1(input, &[4, 3, 2, 1, 0]));
        assert_eq!(43210, run_1(input));

        let input = "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0";
        assert_eq!(54321, run_amp_1(input, &[0, 1, 2, 3, 4]));
        assert_eq!(54321, run_1(input));

        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0";
        assert_eq!(65210, run_amp_1(input, &[1, 0, 4, 3, 2]));
        assert_eq!(65210, run_1(input));
    }

    #[test]
    fn aoc7_2() {
        use super::*;
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        assert_eq!(139629729, run_amp_2(input, &[9, 8, 7, 6, 5]));
        assert_eq!(139629729, run_2(input));

        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        assert_eq!(18216, run_amp_2(input, &[9, 7, 8, 5, 6]));
        assert_eq!(18216, run_2(input));
    }

    #[test]
    fn aoc7_still_correct() {
        use super::*;
        let input = fs::read_to_string("day7.txt").unwrap();
        assert_eq!(20413, run_1(&input));
        assert_eq!(3321777, run_2(&input));
    }
}
