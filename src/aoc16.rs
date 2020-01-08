use std::collections::HashMap;
use std::fs;

use nom::{bytes::complete::take_while1, character::is_digit, combinator::map, IResult};

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();

    println!("16:1 - {}", run_1(&input));
    println!("16:2 - {}", run_2(&input));
}

fn is_char_digit(chr: char) -> bool {
    chr.is_ascii() && is_digit(chr as u8)
}

fn parse(i: &str) -> IResult<&str, Vec<u32>> {
    map(take_while1(is_char_digit), |digits: &str| {
        digits.chars().map(|c| c.to_digit(10).unwrap()).collect()
    })(i)
}

fn run_1(input: &str) -> usize {
    0
}

fn run_2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc16_parse() {
        use super::*;
        assert_eq!(parse("15243"), Ok(("", vec![1, 5, 2, 4, 3])));
    }

    #[test]
    fn aoc16_run_1() {
        use super::*;
        assert_eq!(
            run_1("Before: [3, 2, 1, 1]\n9 2 1 2\nAfter:  [3, 2, 2, 1]\n\n"),
            1
        );
    }
}
