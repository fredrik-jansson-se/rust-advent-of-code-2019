use std::collections::HashMap;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day11.txt").unwrap();
    println!("11:1: {:?}", run_1(&input));
    println!("11:2:\n{:}", run_2(&input));
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Up,
    Down,
    Right,
}

impl Direction {
    fn left(&self) -> Self {
        match self {
            Self::Left => Self::Down,
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn right(&self) -> Self {
        match self {
            Self::Left => Self::Up,
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }

    fn mv(&self, (x, y): (isize, isize)) -> (isize, isize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
        }
    }
}

fn run_1(program: &str) -> usize {
    let mut cpu = crate::intcode::CPU::new(program);
    let mut current_dir = Direction::Up;
    let mut current_pos = (0, 0);
    let mut floor = HashMap::new();

    loop {
        let cur_color = floor.entry(current_pos).or_insert(0);
        let mut input = vec![*cur_color];
        let res = cpu.run(&mut input);

        *cur_color = cpu.output[0];

        match cpu.output[1] {
            0 => current_dir = current_dir.left(),
            1 => current_dir = current_dir.right(),
            _ => unreachable!(),
        };
        cpu.output.resize(0, 0);

        current_pos = current_dir.mv(current_pos);

        if res == crate::intcode::State::Exited {
            break;
        }
    }

    floor.len()
}

fn run_2(program: &str) -> String {
    let mut cpu = crate::intcode::CPU::new(program);
    let mut current_dir = Direction::Up;
    let mut current_pos = (0, 0);
    let mut floor = HashMap::new();

    // Make start floor white
    floor.insert((0, 0), 1);

    loop {
        let cur_color = floor.entry(current_pos).or_insert(0);
        let mut input = vec![*cur_color];
        let res = cpu.run(&mut input);

        *cur_color = cpu.output[0];

        match cpu.output[1] {
            0 => current_dir = current_dir.left(),
            1 => current_dir = current_dir.right(),
            _ => unreachable!(),
        };
        cpu.output.resize(0, 0);

        current_pos = current_dir.mv(current_pos);

        if res == crate::intcode::State::Exited {
            break;
        }
    }

    let mut res = String::new();
    for row in 0..7 {
        for col in 0..40 {
            match floor.get(&(col, row)) {
                Some(c) if *c == 1 => res += "*",
                _ => res += " ",
            }
        }
        res += "\n";
    }

    res
}

#[cfg(test)]
mod tests {
    //
}
