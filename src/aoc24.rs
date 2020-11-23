use std::collections::HashSet;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day24.txt").unwrap();
    println!("24:1 {}", run_1(&input));
    println!("24:2 {}", run_2(&input));
}

fn parse(input: &str) -> (HashSet<(usize, usize)>, usize) {
    let mut res = HashSet::new();
    let mut size = 0;

    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            if c == '#' {
                res.insert((x, y));
            }
        }
        size = y;
    }
    (res, size + 1)
}

fn num_nbr_bugs(bugs: &HashSet<(usize, usize)>, x: usize, y: usize) -> usize {
    let mut num_bugs = 0;
    if x > 0 && bugs.contains(&(x - 1, y)) {
        num_bugs += 1;
    }
    if y > 0 && bugs.contains(&(x, y - 1)) {
        num_bugs += 1;
    }

    if bugs.contains(&(x + 1, y)) {
        num_bugs += 1;
    }

    if bugs.contains(&(x, y + 1)) {
        num_bugs += 1;
    }

    num_bugs
}

fn run_1(input: &str) -> u128 {
    let (mut bugs, size) = parse(input);

    let mut old_bugs = HashSet::new();

    loop {
        let mut bio_div_rating = 0;
        let mut new_bugs = HashSet::new();
        for y in 0..size {
            for x in 0..size {
                let nbrs = num_nbr_bugs(&bugs, x, y);
                if bugs.contains(&(x, y)) && nbrs == 1 {
                    new_bugs.insert((x, y));
                } else if !bugs.contains(&(x, y)) && (nbrs == 1 || nbrs == 2) {
                    new_bugs.insert((x, y));
                }
                if new_bugs.contains(&(x, y)) {
                    let idx = y * size + x;
                    let bdr = 2u128.pow(idx as u32);
                    bio_div_rating += bdr;
                }
            }
        }

        bugs = new_bugs;
        if old_bugs.contains(&bio_div_rating) {
            return bio_div_rating;
        }

        old_bugs.insert(bio_div_rating);
    }
}

fn run_2(input: &str) -> u128 {
    let (mut bugs, size) = parse(input);

    let mut old_bugs = HashSet::new();

    loop {
        println!("bugs:");
        for y in 0..size {
            for x in 0..size {
                match bugs.contains(&(x, y)) {
                    true => print!("#"),
                    false => print!("."),
                }
            }
            println!("");
        }
        println!("");

        let mut bio_div_rating = 0;
        let mut new_bugs = HashSet::new();
        for y in 0..size {
            for x in 0..size {
                let nbrs = num_nbr_bugs(&bugs, x, y);
                if bugs.contains(&(x, y)) && nbrs == 1 {
                    new_bugs.insert((x, y));
                } else if !bugs.contains(&(x, y)) && (nbrs == 1 || nbrs == 2) {
                    new_bugs.insert((x, y));
                }
                if new_bugs.contains(&(x, y)) {
                    let idx = y * size + x;
                    println!("{:?} -> {}", (x, y), idx);
                    let bdr = 2u128.pow(idx as u32);
                    bio_div_rating += bdr;
                }
            }
        }

        bugs = new_bugs;
        if old_bugs.contains(&bio_div_rating) {
            println!("bugs:");
            for y in 0..size {
                for x in 0..size {
                    match bugs.contains(&(x, y)) {
                        true => print!("#"),
                        false => print!("."),
                    }
                }
                println!("");
            }
            println!("");
            return bio_div_rating;
        }

        old_bugs.insert(bio_div_rating);
    }
}

mod tests {
    #[test]
    fn aoc24_parse() {
        use super::*;
        let state = "....#
#..#.
#..##
..#..
#...";
        let (bugs, size) = parse(state);

        assert_eq!(size, 5);

        assert!(bugs.contains(&(4, 0)));
        assert!(bugs.contains(&(0, 4)));
    }

    #[test]
    fn aoc24_run_1() {
        let state = "....#
#..#.
#..##
..#..
#...";
        assert_eq!(super::run_1(state), 2129920);
    }
}
