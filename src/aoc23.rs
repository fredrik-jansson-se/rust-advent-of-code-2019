use crate::intcode::CPU;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day23.txt").unwrap();
    println!("23:1 {}", run_1(&input));
    println!("23:2 {}", run_2(&input));
}

fn run_1(program: &str) -> i64 {
    let mut cpus: Vec<CPU> = (0..50)
        .into_iter()
        .map(|i| {
            let mut cpu = CPU::new(program);
            let mut input = vec![i];
            cpu.run(&mut input);
            cpu
        })
        .collect();

    let mut queues: HashMap<usize, Vec<i64>> = HashMap::new();

    loop {
        for (i, cpu) in cpus.iter_mut().enumerate() {
            // Gather output and store in queue
            for (oi, addr) in cpu.output.iter().enumerate().step_by(3) {
                let queue = queues.entry(*addr as usize).or_insert(Vec::new());
                queue.push(cpu.output[oi + 1]);
                queue.push(cpu.output[oi + 2]);
                if *addr == 255 {
                    return cpu.output[oi + 2];
                }
            }
            cpu.output.clear();

            let queue = queues.entry(i).or_insert(Vec::new());
            if queue.len() > 0 {
                cpu.run(queue);
            } else {
                cpu.run(&mut vec![-1]);
            }
        }
    }
}

fn run_2(program: &str) -> i64 {
    let mut cpus: Vec<CPU> = (0..50)
        .into_iter()
        .map(|i| {
            let mut cpu = CPU::new(program);
            let mut input = vec![i];
            cpu.run(&mut input);
            cpu
        })
        .collect();

    let mut queues: HashMap<usize, Vec<i64>> = HashMap::new();
    let mut nat_vals = HashSet::new();
    loop {
        let mut all_idle = true;
        for (i, cpu) in cpus.iter_mut().enumerate() {
            // Gather output and store in queue
            for (oi, addr) in cpu.output.iter().enumerate().step_by(3) {
                let queue = queues.entry(*addr as usize).or_insert(Vec::new());
                // Only store last values in the NAT
                if *addr == 255 {
                    queue.clear();
                }
                queue.push(cpu.output[oi + 1]);
                queue.push(cpu.output[oi + 2]);
            }
            cpu.output.clear();

            let queue = queues.entry(i).or_insert(Vec::new());
            if queue.len() > 0 {
                cpu.run(queue);
                all_idle = false;
            } else {
                cpu.run(&mut vec![-1]);
            }
        }

        if all_idle {
            match queues.get_mut(&255) {
                Some(nat) if nat.len() > 0 => {
                    let y = nat[1];
                    if !nat_vals.insert(y) {
                        return y;
                    }
                    cpus[0].run(nat);
                }
                _ => (),
            }
        }
    }
}
