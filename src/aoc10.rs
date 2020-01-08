use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day10.txt").unwrap();
    println!("10:1 {:?}", run_1(&input));
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else if a < b {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}

fn run_1(input: &str) -> (usize, usize) {
    let map = parse(input);
    let mut lines = HashSet::new();
    let mut asteroids = HashSet::new();

    for (source_y, source_row) in map.iter().enumerate() {
        for (source_x, s) in source_row.iter().enumerate() {
            if !s {
                continue;
            }
            asteroids.insert((source_x, source_y));
            for (dest_y, dest_row) in map.iter().enumerate().skip(source_y) {
                // if we are on the same line, just look forward in the x-dir
                let x_skip = if dest_y == source_y { source_x + 1 } else { 0 };
                for (dest_x, d) in dest_row.iter().enumerate().skip(x_skip) {
                    if !d {
                        continue;
                    }
                    asteroids.insert((dest_x, dest_y));
                    lines.insert((source_x, source_y, dest_x, dest_y));
                    // dbg! {(source_x, source_y, dest_x, dest_y)};
                }
            }
        }
    }
    for (x1, y1, x2, y2) in lines {
        let mut dy = y2 as i32 - y1 as i32;
        let mut dx = x2 as i32 - x1 as i32;
        let g = gcd(dy.abs(), dx.abs());
        dy /= g;
        dx /= g;

        dbg! {(dx, dy)};
    }
    (0, 0)
}

fn parse(i: &str) -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for line in i.lines() {
        let row = line.chars().map(|c| c == '#').collect();

        map.push(row);
    }
    map
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc10_parse() {
        use super::*;
        let map = parse(
            r#".#..#
.....
#####
....#
...##"#,
        );
        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 5);
        assert!(map[2][0]);
        assert!(!map[3][0]);
    }

    #[test]
    fn aoc10_run_1() {
        use super::*;
        assert_eq!(
            run_1(
                r#".#..#
.....
#####
....#
...##"#
            ),
            (3, 4)
        );
    }
}
