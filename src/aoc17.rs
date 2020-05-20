use std::fs;

#[derive(Debug, PartialEq)]
enum MapItem {
    RobotUp,
    RobotDown,
    RobotRight,
    RobotLeft,
    RobotDead,
    Scaffold,
    Empty,
}

impl MapItem {
    fn parse(c: char) -> Self {
        match c {
            '^' => Self::RobotUp,
            'v' => Self::RobotDown,
            '>' => Self::RobotRight,
            '<' => Self::RobotLeft,
            'X' => Self::RobotDead,
            '#' => Self::Scaffold,
            _ => Self::Empty,
        }
    }
}

fn parse(input: &str) -> Vec<Vec<MapItem>> {
    let mut res = Vec::new();
    for r in input.lines().filter(|l| l.len() > 0) {
        let row = r.chars().map(MapItem::parse).collect();
        res.push(row)
    }
    res
}

pub fn run() {
    let input = fs::read_to_string("day17.txt").unwrap();

    println!("day17-1: {}", run_1(&input));
    //
}

fn count_intersections(map: &[Vec<MapItem>]) -> usize {
    let mut s = 0;

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != MapItem::Scaffold {
                continue;
            }

            let left = if x > 0 { Some(&map[y][x - 1]) } else { None };
            let right = if x < (map[y].len() - 1) {
                Some(&map[y][x + 1])
            } else {
                None
            };

            let above = if y > 0 { Some(&map[y - 1][x]) } else { None };
            let below = if y < (map.len() - 1) {
                Some(&map[y + 1][x])
            } else {
                None
            };

            let nbrs = [left, right, above, below];
            if nbrs
                .iter()
                .filter(|v| **v == Some(&MapItem::Scaffold))
                .count()
                > 2
            {
                s += x * y;
            }
        }
    }

    s
}

fn run_1(input: &str) -> usize {
    let mut cpu = crate::intcode::CPU::new(input);

    cpu.run(&mut vec![]);
    let camera = String::from_utf8(cpu.output.iter().map(|i| *i as u8).collect()).unwrap();
    let map = parse(&camera);
    count_intersections(&map)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc17_parse() {
        use super::*;
        let map = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";
        let map = parse(map);

        assert_eq!(map[6][10], MapItem::RobotUp);
    }

    #[test]
    fn aoc17_run_1() {
        use super::*;
        let map = "..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..";
        let map = parse(map);
        assert_eq!(count_intersections(&map), 76);
    }
}
