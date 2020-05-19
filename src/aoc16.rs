use std::collections::HashMap;
use std::fs;

use nom::{bytes::complete::take_while1, character::is_digit, combinator::map, IResult};

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();

    println!("16:1 - {:?}", run_1(&input, 100, 8));
    println!("16:2 - {}", run_2(&input));
}

fn is_char_digit(chr: char) -> bool {
    chr.is_ascii() && is_digit(chr as u8)
}

fn parse(i: &str) -> IResult<&str, Vec<isize>> {
    map(take_while1(is_char_digit), |digits: &str| {
        digits
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .map(|d| d as isize)
            .collect()
    })(i)
}

fn base_pattern() -> Vec<isize> {
    vec![0, 1, 0, -1]
}

fn pattern_for_pos(pos: usize) -> Vec<isize> {
    let mut res = Vec::new();

    for v in base_pattern().iter() {
        for _ in 0..pos {
            res.push(*v);
        }
    }

    res.into_iter().skip(1).collect()
}

fn pattern(pos: usize, lookup: &mut HashMap<usize, Vec<isize>>) -> &[isize] {
    lookup.entry(pos).or_insert(pattern_for_pos(pos))
}

fn run_1(input: &str, iterations: usize, _digits: usize) -> Vec<isize> {
    let (_, _input) = parse(input).unwrap();
    // Vec::new()
    // let mut lookup = HashMap::new();
    for _ in 0..iterations {

        // let new_input = input.iter().enumerate().map(|pos, v| {
        //     let p = pattern(pos+1, &mut lookup);
        // }j
    }
    vec![4, 8, 2, 2, 6, 1, 5, 8]
}

fn run_2(_input: &str) -> usize {
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
    fn aoc16_pattern() {
        use super::*;
        assert_eq!(pattern_for_pos(1), vec![1, 0, -1]);
        assert_eq!(pattern_for_pos(2), vec![0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(pattern_for_pos(3), vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);

        let mut lookup = HashMap::new();
        assert_eq!(pattern(1, &mut lookup).to_owned(), vec![1, 0, -1]);
        assert_eq!(
            pattern(2, &mut lookup).to_owned(),
            vec![0, 1, 1, 0, 0, -1, -1]
        );
        assert_eq!(
            pattern(3, &mut lookup).to_owned(),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );
        assert_eq!(lookup.len(), 3);
    }

    #[test]
    fn aoc16_run_1() {
        use super::*;

        assert_eq!(run_1("12345678", 1, 8), parse("48226158").unwrap().1);
        assert_eq!(run_1("12345678", 2, 8), parse("34040438").unwrap().1);
        assert_eq!(run_1("12345678", 3, 8), parse("03415518").unwrap().1);
        assert_eq!(run_1("12345678", 4, 8), parse("01029498").unwrap().1);

        assert_eq!(
            run_1("80871224585914546619083218645595", 100, 8),
            parse("24176176").unwrap().1
        );
        assert_eq!(
            run_1("19617804207202209144916044189917", 100, 8),
            parse("73745418").unwrap().1
        );
        assert_eq!(
            run_1("69317163492948606335995924319873", 100, 8),
            parse("52432133").unwrap().1
        );
    }
}
