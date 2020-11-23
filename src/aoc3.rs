use super::helper::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day3.txt").unwrap();
    let (_, wires) = parse(&input).unwrap();

    println!("3:1 - {}", run_1(&wires));
    println!("3:2 - {}", run_2(&wires));
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up(usize),
    Down(usize),
    Right(usize),
    Left(usize),
}

fn parse_wire(i: &str) -> IResult<&str, Vec<Direction>> {
    let up = map(preceded(tag("U"), usize_val), Direction::Up);
    let down = map(preceded(tag("D"), usize_val), Direction::Down);
    let left = map(preceded(tag("L"), usize_val), Direction::Left);
    let right = map(preceded(tag("R"), usize_val), Direction::Right);
    separated_list1(tag(","), alt((up, down, left, right)))(i)
}

fn parse(i: &str) -> IResult<&str, Vec<Vec<Direction>>> {
    separated_list1(newline, parse_wire)(i)
}

#[derive(Clone, Debug, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn move_in_dir(&self, d: &Direction) -> Self {
        match d {
            Direction::Up(d) => Point {
                x: self.x,
                y: self.y - *d as isize,
            },
            Direction::Down(d) => Point {
                x: self.x,
                y: self.y + *d as isize,
            },
            Direction::Left(d) => Point {
                x: self.x - *d as isize,
                y: self.y,
            },
            Direction::Right(d) => Point {
                x: self.x + *d as isize,
                y: self.y,
            },
        }
    }

    fn distance(&self, o: &Self) -> isize {
        (self.x - o.x).abs() + (self.y - o.y).abs()
    }
}

#[derive(Clone, Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn x_min(&self) -> isize {
        isize::min(self.p1.x, self.p2.x)
    }

    fn x_max(&self) -> isize {
        isize::max(self.p1.x, self.p2.x)
    }

    fn y_min(&self) -> isize {
        isize::min(self.p1.y, self.p2.y)
    }

    fn y_max(&self) -> isize {
        isize::max(self.p1.y, self.p2.y)
    }

    fn len(&self) -> isize {
        (self.p1.x - self.p2.x).abs() + (self.p1.y - self.p2.y).abs()
    }

    fn intersect(&self, o: &Line) -> Option<Point> {
        if self.is_vertical() == o.is_vertical() {
            None
        } else {
            let (v, h) = if self.is_vertical() {
                (self, o)
            } else {
                (o, self)
            };
            if v.p1.x > h.x_min() && v.p1.x < h.x_max() && h.p1.y > v.y_min() && h.p1.y < v.y_max()
            {
                Some(Point {
                    x: v.p1.x,
                    y: h.p2.y,
                })
            } else {
                None
            }
        }
    }
}

fn wire_dir_to_lines(mut start: Point, dirs: &[Direction]) -> Vec<Line> {
    let mut lines = Vec::new();

    for d in dirs {
        let nxt = start.move_in_dir(&d);
        lines.push(Line::new(start.clone(), nxt.clone()));

        start = nxt;
    }

    lines
}

fn run_1(wires: &[Vec<Direction>]) -> isize {
    let lines = wires
        .iter()
        .map(|w| wire_dir_to_lines(Point { x: 0, y: 0 }, w))
        .collect::<Vec<Vec<Line>>>();
    let mut closest = None;

    for l1 in &lines[0] {
        for l2 in &lines[1] {
            match l1.intersect(&l2) {
                None => (),
                Some(p) => {
                    let d = p.x.abs() + p.y.abs();
                    match closest {
                        Some(o) if o > d => closest = Some(d),
                        None => closest = Some(d),
                        _ => (),
                    }
                }
            }
        }
    }

    closest.unwrap()
}

fn run_2(wires: &[Vec<Direction>]) -> isize {
    let lines = wires
        .iter()
        .map(|w| wire_dir_to_lines(Point { x: 0, y: 0 }, w))
        .collect::<Vec<Vec<Line>>>();

    let mut l1_len = 0;

    let mut min_len = std::isize::MAX;

    for l1 in &lines[0] {
        let mut l2_len = 0;
        for l2 in &lines[1] {
            match l1.intersect(&l2) {
                None => (),
                Some(p) => {
                    min_len = isize::min(
                        l1_len + l2_len + l1.p1.distance(&p) + l2.p1.distance(&p),
                        min_len,
                    );
                }
            }

            l2_len += l2.len();
        }
        l1_len += l1.len();
    }

    min_len
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc3_intersect() {
        use super::*;
        let l1 = Line {
            p1: Point { x: 3, y: -5 },
            p2: Point { x: 3, y: -2 },
        };
        let l2 = Line {
            p1: Point { x: 6, y: -3 },
            p2: Point { x: 2, y: -3 },
        };

        assert_eq!(Some(Point { x: 3, y: -3 }), l1.intersect(&l2));
    }
    #[test]
    fn aoc3_parse() {
        use super::*;
        let (_, wires) =
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        assert_eq!(2, wires.len());
        let wire = &wires[0];
        assert_eq!(9, wire.len());
        assert_eq!(Direction::Right(75), wire[0]);
        assert_eq!(Direction::Down(30), wire[1]);
        assert_eq!(Direction::Up(83), wire[3]);
        assert_eq!(Direction::Left(12), wire[4]);
    }

    #[test]
    fn aoc3_run_1() {
        use super::*;
        let (_, wires) = parse("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap();
        assert_eq!(6, run_1(&wires));

        let (_, wires) =
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        assert_eq!(159, run_1(&wires));

        let (_, wires) = parse(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        assert_eq!(135, run_1(&wires));
    }

    #[test]
    fn aoc3_run_2() {
        use super::*;
        let (_, wires) = parse("R8,U5,L5,D3\nU7,R6,D4,L4").unwrap();
        assert_eq!(30, run_2(&wires));

        let (_, wires) =
            parse("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83").unwrap();
        assert_eq!(610, run_2(&wires));

        let (_, wires) = parse(
            "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
        )
        .unwrap();
        assert_eq!(410, run_2(&wires));
    }
}
