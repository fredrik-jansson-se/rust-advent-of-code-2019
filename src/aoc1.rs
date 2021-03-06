use super::helper::*;
use nom::character::complete::newline;
use nom::multi::separated_list1;
use nom::*;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day1.txt").unwrap();
    println!("1:1: {}", run_1(&input));
    println!("1:2: {}", run_2(&input));
}

fn fuel_1(mass: &usize) -> usize {
    mass / 3 - 2
}

fn fuel_2(mass: &usize) -> usize {
    let mut fuel = 0;
    let mut l_mass = *mass;

    while l_mass > 6 {
        let dfuel = fuel_1(&l_mass);

        fuel += dfuel;

        l_mass = dfuel;
    }

    fuel
}

fn parse_input(i: &str) -> IResult<&str, Vec<usize>> {
    separated_list1(newline, usize_val)(i)
}

fn run_1(input: &str) -> usize {
    let (_, masses) = parse_input(input).unwrap();
    masses.iter().map(fuel_1).sum()
}

fn run_2(input: &str) -> usize {
    let (_, masses) = parse_input(input).unwrap();
    masses.iter().map(fuel_2).sum()
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc1_fuel_1() {
        use super::*;
        assert_eq!(2, fuel_1(&12));
        assert_eq!(2, fuel_1(&14));
        assert_eq!(654, fuel_1(&1969));
        assert_eq!(33583, fuel_1(&100756));
    }

    #[test]
    fn aoc1_fuel_2() {
        use super::*;
        assert_eq!(2, fuel_2(&14));
        assert_eq!(966, fuel_2(&1969));
        assert_eq!(50346, fuel_2(&100756));
    }
}
