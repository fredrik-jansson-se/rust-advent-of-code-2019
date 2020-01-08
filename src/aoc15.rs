use pathfinding::prelude::astar;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day15.txt").unwrap();
    println!("15:1 {}", run_1(&input));
    // 351 too high
    println!("15:2 {}", run_2(&input));
}

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn to_opcode(&self) -> i64 {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }

    fn to_direction((from_x, from_y): Coord, (to_x, to_y): Coord) -> Self {
        match (to_x - from_x, to_y - from_y) {
            (-1, 0) => Self::West,
            (1, 0) => Self::East,
            (0, -1) => Self::North,
            (0, 1) => Self::South,
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum MoveResult {
    Moved,
    HitWall,
    MovedAndOxygen,
}

type Coord = (i64, i64);

fn neighbors((x, y): Coord, visited: &HashSet<Coord>) -> Vec<Coord> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
        .into_iter()
        .filter(|c| !visited.contains(c))
        .collect()
}

fn do_move(cpu: &mut crate::intcode::CPU, from: Coord, to: Coord) -> MoveResult {
    let dir = Direction::to_direction(from, to);
    let mut input = vec![dir.to_opcode()];
    cpu.run(&mut input);

    match cpu.output.pop() {
        Some(0) => MoveResult::HitWall,
        Some(1) => MoveResult::Moved,
        Some(2) => MoveResult::MovedAndOxygen,
        _ => unreachable!(),
    }
}

fn build_map(program: &str) -> (HashMap<Coord, bool>, Coord) {
    let mut cpu = crate::intcode::CPU::new(program);
    let mut cur = (0, 0);
    let mut oxy_pos = None;

    let mut map = HashMap::new();
    let mut visited = HashSet::new();
    let mut path = Vec::new();

    loop {
        visited.insert(cur);
        map.insert(cur, true);
        let nbrs = neighbors(cur, &visited);

        // Try to get the first neighbor

        match nbrs.iter().next() {
            Some(to) => match do_move(&mut cpu, cur, *to) {
                MoveResult::HitWall => {
                    visited.insert(*to);
                    map.insert(*to, false);
                }
                r => {
                    if r == MoveResult::MovedAndOxygen {
                        oxy_pos = Some(*to);
                    }

                    path.push(cur);
                    cur = *to;
                }
            },
            None => {
                // backtrack to nearest to_explore
                match path.pop() {
                    // Go back one step in the path
                    Some(to) => {
                        // We ignore the result here as we're moving back
                        // println!("Backtrack from {:?} to {:?}", cur, to);
                        do_move(&mut cpu, cur, to);
                        cur = to;
                    }
                    // If path is empty, we've exhausted all search options and the map should be
                    // complete
                    None => break,
                }
            }
        }
    }

    (map, oxy_pos.unwrap())
}

fn run_1(program: &str) -> usize {
    let (map, oxy_pos) = build_map(program);

    let (min_x, min_y, max_x, max_y) =
        map.keys()
            .fold((0, 0, 0, 0), |(min_x, min_y, max_x, max_y), (x, y)| {
                (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
            });

    let start = (0, 0);

    // Print map
    let f = false;
    for y in min_y..max_y {
        for x in min_x..max_x {
            if *map.get(&(x, y)).unwrap_or(&f) {
                if oxy_pos == (x, y) {
                    print!("O");
                } else if start == (x, y) {
                    print!("S");
                } else {
                    print!(".");
                }
            } else {
                print!("#");
            }
        }
        println!("");
    }

    // Find the shortest path from start to oxy_pos
    let shortest_path = astar(
        &start,
        |(x, y)| {
            vec![(*x - 1, *y), (*x + 1, *y), (*x, *y - 1), (*x, *y + 1)]
                .into_iter()
                .filter(|c| *map.get(c).unwrap())
                .map(|c| (c, 1))
                .collect::<Vec<(Coord, i64)>>()
        },
        |(x, y)| (x - oxy_pos.0).abs() + (y - oxy_pos.1).abs(),
        |c| *c == oxy_pos,
    );
    // Number of steps are the elements in shortest_path - 1 (since start is included)
    shortest_path.unwrap().0.len() - 1
}

fn run_2(program: &str) -> usize {
    let (map, oxy_pos) = build_map(program);

    let mut oxygen_filled = HashSet::new();
    oxygen_filled.insert(oxy_pos);

    let mut neighbors = HashSet::new();
    neighbors.insert(oxy_pos);

    let mut minutes = 0;

    while !neighbors.is_empty() {
        let mut new_nbrs = HashSet::new();
        for (x, y) in neighbors.iter() {
            // Make sure we don't consider this again
            oxygen_filled.insert((*x, *y));

            vec![(*x - 1, *y), (*x + 1, *y), (*x, *y - 1), (*x, *y + 1)]
                .into_iter()
                .filter(|c| *map.get(c).unwrap())
                .filter(|c| !oxygen_filled.contains(c))
                .collect::<Vec<_>>()
                .iter()
                .for_each(|c| {
                    new_nbrs.insert(*c);
                });
        }

        neighbors = new_nbrs;
        if !neighbors.is_empty() {
            minutes += 1;
        }
    }

    minutes
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc15_run_2() {
        // let map =
    }
}
