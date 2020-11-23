use crate::helper::*;
use std::fs;

use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    IResult,
};

#[derive(Clone, Debug, PartialEq)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

fn parse_vec3(i: &str) -> IResult<&str, Vec3> {
    let (i, _) = tag("<x=")(i)?;
    let (i, x) = i32_val(i)?;
    let (i, _) = tag(", y=")(i)?;
    let (i, y) = i32_val(i)?;
    let (i, _) = tag(", z=")(i)?;
    let (i, z) = i32_val(i)?;
    let (i, _) = tag(">")(i)?;
    Ok((i, Vec3 { x, y, z }))
}

#[derive(Clone, PartialEq)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

fn pull(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (-1, 1)
    } else if a < b {
        (1, -1)
    } else {
        (0, 0)
    }
}

impl Moon {
    fn new(pos: Vec3) -> Self {
        Moon {
            pos,
            vel: Vec3 { x: 0, y: 0, z: 0 },
        }
    }

    fn gravity(&mut self, o: &mut Self) {
        let (d, od) = pull(self.pos.x, o.pos.x);
        self.vel.x += d;
        o.vel.x += od;
        let (d, od) = pull(self.pos.y, o.pos.y);
        self.vel.y += d;
        o.vel.y += od;
        let (d, od) = pull(self.pos.z, o.pos.z);
        self.vel.z += d;
        o.vel.z += od;
    }

    fn step_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }

    fn potential_energy(&self) -> i32 {
        self.pos.x.abs() + self.pos.y.abs() + self.pos.z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        self.vel.x.abs() + self.vel.y.abs() + self.vel.z.abs()
    }
}

fn parse(i: &str) -> IResult<&str, Vec<Moon>> {
    separated_list1(newline, map(parse_vec3, Moon::new))(i)
}

pub fn run() {
    let input = fs::read_to_string("day12.txt").unwrap();
    println!("12:1 {}", run_1(&input, 1000));
    println!("12:2 {}", run_2(&input));
}

fn run_1(input: &str, steps: usize) -> i32 {
    let (_, mut moons) = parse(input).unwrap();
    for _ in 0..steps {
        for a in 0..moons.len() {
            let (left, right) = moons.split_at_mut(a + 1);
            for mb in right {
                left[a].gravity(mb);
            }
            left[a].step_velocity();
        }
    }

    moons
        .iter()
        .map(|m| m.kinetic_energy() * m.potential_energy())
        .sum()
}

fn run_2(input: &str) -> u64 {
    let (_, mut moons) = parse(input).unwrap();
    let moons_orig = moons.clone();

    // Count the number of steps until moon is back at original spot
    let mut moon_steps = vec![0; moons.len()];

    // Stop counting when moon is back
    let mut moon_done = vec![false; moons.len()];
    while !moon_done.iter().all(|&t| t) {
        for a in 0..moons.len() {
            let (left, right) = moons.split_at_mut(a + 1);
            for mb in right {
                left[a].gravity(mb);
            }
            left[a].step_velocity();
            if !moon_done[a] {
                moon_steps[a] += 1;
                if left[a] == moons_orig[a] {
                    println!("{} done", a);
                    moon_done[a] = true;
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc12_parse() {
        use super::*;
        let input = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;
        let (_, moons) = parse(input).unwrap();
        assert_eq!(moons.len(), 4);
    }

    #[test]
    fn aoc12_run_1() {
        use super::*;
        let input = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;
        assert_eq!(run_1(input, 10), 179);

        let input = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#;
        assert_eq!(run_1(input, 100), 1940);
    }

    #[test]
    fn aoc12_run_2() {
        use super::*;
        let input = r#"<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>"#;
        assert_eq!(run_2(input), 2772);

        let input = r#"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>"#;
        assert_eq!(run_2(input), 4686774924);
    }
}
