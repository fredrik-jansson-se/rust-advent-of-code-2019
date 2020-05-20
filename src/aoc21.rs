use std::fs;

pub fn run() {
    let input = fs::read_to_string("day21.txt").unwrap();
    println!("21:1 {:?}", run_1(&input));
}

fn run_1(program: &str) -> i64 {
    let mut cpu = crate::intcode::CPU::new(program);

    cpu.run(&mut vec![]);
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc21_parse() {}
}
