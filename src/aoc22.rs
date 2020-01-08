use pathfinding::prelude::{absdiff, astar};

type Coord = (usize, usize);

pub fn run() {
    println!("22:1 {}", run_1(11991, (6, 797)));
    println!("22:2 {}", run_2(11991, (6, 797)));
    // 1051
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

#[derive(Debug, PartialEq, Clone)]
enum Type {
    //    Unknown,
    Rocky,
    Wet,
    Narrow,
}

impl Type {
    fn from_erosion_level(erosion_level: u64) -> Self {
        match erosion_level % 3 {
            0 => Type::Rocky,
            1 => Type::Wet,
            _ => Type::Narrow,
        }
    }

    fn available_tools(&self) -> &'static [Tool] {
        match self {
            Type::Rocky => &[Tool::Torch, Tool::ClimbingGear],
            Type::Wet => &[Tool::ClimbingGear, Tool::Neither],
            Type::Narrow => &[Tool::Torch, Tool::Neither],
        }
    }
}

type Map = Vec<Vec<u64>>;

fn create_map(width: usize, height: usize) -> Map {
    let mut res = Vec::with_capacity(height);

    for _ in 0..height {
        res.push(vec![0; width]);
    }

    res
}

fn geologic_index(x: usize, y: usize, target: &Coord, map: &Map) -> u64 {
    if x == 0 && y == 0 {
        0
    } else if x == target.0 && y == target.1 {
        0
    } else if x == 0 {
        y as u64 * 48271
    } else if y == 0 {
        x as u64 * 16807
    } else {
        map[y - 1][x] * map[y][x - 1]
    }
}

fn erosion_level(x: usize, y: usize, target: &Coord, map: &Map, depth: usize) -> u64 {
    (geologic_index(x, y, target, map) + depth as u64) % 20183
}

fn expand_map(map: &mut Map, target: &Coord, depth: usize) {
    let old_height = map.len();
    let new_height = 4 * old_height;
    let old_width = map[0].len();
    let new_width = 4 * old_width;
    *map = create_map(new_width, new_height);
    for y in 0..new_height {
        for x in 0..new_width {
            let gl = erosion_level(x, y, &target, &map, depth);
            map[y][x] = gl;
        }
    }
}

fn run_1(depth: usize, target: Coord) -> u64 {
    let width = target.0 + 1;
    let height = target.1 + 1;
    let mut map = create_map(width, height);

    let mut sum = 0;
    for y in 0..height {
        for x in 0..width {
            let gl = erosion_level(x, y, &target, &map, depth);
            map[y][x] = gl;
            sum += gl % 3;
        }
    }

    sum
}

#[derive(Debug)]
struct State {
    target: Coord,
    depth: usize,
    tool: Tool,
    map: Map,
}

impl State {
    fn new(depth: usize, target: Coord) -> Self {
        let width = 1 * target.0 + 1;
        let height = 1 * target.1 + 1;
        let mut map = create_map(width, height);

        for y in 0..height {
            for x in 0..width {
                let gl = erosion_level(x, y, &target, &map, depth);
                map[y][x] = gl;
            }
        }
        State {
            target,
            depth,
            tool: Tool::Torch,
            map,
        }
    }
    fn next_region(&mut self, node: &Node) -> Vec<(Node, usize)> {
        let x = node.pos.0;
        let y = node.pos.1;
        let mut regions = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            regions.push((x - 1, y));
        }
        if y > 0 {
            regions.push((x, y - 1));
        }

        let mut res = Vec::new();

        let cur_widht = self.map[0].len();
        let cur_height = self.map.len();
        if (x + 1) >= cur_widht || (y + 1) > cur_height {
            println!("Expanding map");
            expand_map(&mut self.map, &self.target, self.depth);
        }

        let my_type = Type::from_erosion_level(self.map[node.pos.1][node.pos.0]);
        let my_possible_tools = my_type.available_tools();
        for (x, y) in regions {
            let reg_type = Type::from_erosion_level(self.map[y][x]);
            let reg_possible_tools = reg_type.available_tools();

            for my_tool in my_possible_tools {
                if reg_possible_tools.contains(my_tool) {
                    let cost = if *my_tool == node.tool { 1 } else { 7 + 1 };
                    // Have to enter the target holding a torch
                    if (x, y) == self.target && *my_tool != Tool::Torch {
                        continue;
                    }
                    res.push((
                        Node {
                            pos: (x, y),
                            tool: my_tool.clone(),
                        },
                        cost,
                    ));
                }
            }
        }

        res
    }

    fn distance_to_target(n: &Node, target: &Coord) -> usize {
        (absdiff(n.pos.0, target.0) + absdiff(n.pos.1, target.1))
    }

    fn solve(&mut self) -> Option<(Vec<Node>, usize)> {
        let start = Node {
            pos: (0, 0),
            tool: Tool::Torch,
        };

        let target = self.target;
        let res = astar(
            &start,
            |n| self.next_region(n),
            |n| State::distance_to_target(n, &target),
            |n| n.pos == target,
        );

        // dbg! { &res };
        res
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    pos: Coord,
    tool: Tool,
}

fn run_2(depth: usize, target: (usize, usize)) -> usize {
    let mut state = State::new(depth, target);
    let res = state.solve().unwrap();
    res.1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn aoc22_geologic_index() {
        let map = create_map(11, 11);
        let target = (10, 10);
        assert_eq!(geologic_index(0, 0, &target, &map), 0);
        assert_eq!(geologic_index(10, 10, &target, &map), 0);
    }

    #[test]
    fn aoc22_expand_map() {
        let old_hw = 10;
        let mut map = create_map(old_hw, old_hw);
        assert_eq!(map.len(), old_hw);
        assert_eq!(map[0].len(), old_hw);
        expand_map(&mut map, &(10, 10), 510);
        let new_hw = 4 * old_hw;
        assert_eq!(map.len(), new_hw);
        assert_eq!(map[0].len(), new_hw);
        assert_eq!(map[10].len(), new_hw);
    }

    #[test]
    fn aoc22_test_1() {
        assert_eq!(run_1(510, (10, 10)), 114);
    }

    #[test]
    fn aoc22_test_2() {
        assert_eq!(run_2(510, (10, 10)), 45);
    }
}
