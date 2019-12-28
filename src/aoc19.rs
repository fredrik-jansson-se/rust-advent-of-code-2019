use rayon::prelude::*;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day19.txt").unwrap();
    println!("19:1 - {}", run_1(&input));
    println!("19:2 - {}", run_2(&input, 100));
}

fn runner_1(mut cpu: crate::intcode::CPU, x: i64, y: i64) -> i64 {
    let mut input = vec![x, y];

    cpu.run(&mut input);

    cpu.output[0]
}

fn run_1(input: &str) -> i64 {
    let xs: Vec<i64> = (0..50).collect();
    let ys = xs.clone();

    let cpu = crate::intcode::CPU::new(input);

    xs.par_iter()
        .map(|x: &i64| {
            ys.par_iter()
                .map(|y| {
                    //
                    runner_1(cpu.clone(), *x, *y)
                })
                .sum::<i64>()
        })
        .sum()
}

fn is_valid(cpu: crate::intcode::CPU, x: i64, y: i64) -> bool {
    runner_1(cpu, x, y) == 1
}

fn run_2(input: &str, width: i64) -> i64 {
    let cpu = crate::intcode::CPU::new(input);

    // start wide of the beam
    let mut cur_x = 0;

    // First find left edge on cur_y, we need to do this as there are some empty rows in the
    // beginning
    let mut cur_y = 10;

    loop {
        cur_y += 1;

        // Follow the left edge
        while !is_valid(cpu.clone(), cur_x, cur_y) {
            cur_x += 1;
        }

        // Make sure we're all the way to the left
        while is_valid(cpu.clone(), cur_x - 1, cur_y) {
            cur_x -= 1;
        }

        // If the top_right corner is ok, we're good
        let top_right = (cur_x + width - 1, cur_y - width + 1);
        if !is_valid(cpu.clone(), top_right.0, top_right.1) {
            continue;
        }

        // we're bottom left corner, get coord of top left corner
        return cur_x * 10000 + (cur_y - width + 1);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc19_is_valid() {
        use super::*;
        let input = fs::read_to_string("day19.txt").unwrap();
        let cpu = crate::intcode::CPU::new(&input);

        assert!(is_valid(cpu.clone(), 0, 0));
        assert!(!is_valid(cpu.clone(), 0, 1));
        // for y in 0..20 {
        //     for x in 0..20 {
        //         if runner_1(cpu.clone(), x, y) == 1 {
        //             print!("#");
        //         } else {
        //             print!(".");
        //         }
        //     }
        //     println!("");
        // }
        assert!(is_valid(cpu.clone(), 5, 3));
        assert!(is_valid(cpu.clone(), 7, 4));
    }
    #[test]
    fn aoc19_run_2() {
        use super::*;
        let input = fs::read_to_string("day19.txt").unwrap();
        assert_eq!(run_2(&input, 10), 25 * 10000 + 20);
    }
}
