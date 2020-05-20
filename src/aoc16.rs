use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;

use nom::{bytes::complete::take_while1, character::is_digit, combinator::map, IResult};

pub fn run() {
    let input = fs::read_to_string("day16.txt").unwrap();

    println!("16:1 - {:?}", run_1(&input, 100, 8));
    println!("16:2 - {:?}", run_2(&input, 100, 8));
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

const BASE_PATTERN: [isize; 4] = [0, 1, 0, -1];

fn pattern_for_pos(pos: usize, input_len: usize) -> Vec<isize> {
    let mut res = Vec::with_capacity(pos * BASE_PATTERN.len());

    while res.len() < input_len {
        for v in BASE_PATTERN.iter() {
            for _ in 0..pos {
                res.push(*v);
            }
        }
    }

    res.into_iter().skip(1).take(input_len).collect()
}

fn pattern(pos: usize, input_len: usize, lookup: &mut HashMap<usize, Vec<isize>>) -> &[isize] {
    lookup.entry(pos).or_insert(pattern_for_pos(pos, input_len))
}

fn run_1(input: &str, iterations: usize, digits: usize) -> Vec<isize> {
    let (_, mut input) = parse(input).unwrap();

    let mut lookup = HashMap::new();

    for _ in 0..iterations {
        let new_input = input
            .iter()
            .enumerate()
            .map(|(pos, _)| {
                let p = pattern(pos + 1, input.len(), &mut lookup).iter();
                let muls = input.iter().zip(p).map(|(a, b)| a * b);
                let sum = muls.sum::<isize>().abs();
                sum % 10
            })
            .collect();

        input = new_input;
    }
    input.truncate(digits);
    input
}

fn run_2(input: &str, iterations: usize, digits: usize) -> Vec<isize> {
    let (_, mut input) = parse(input).unwrap();

    let input_len = input.len() * 10000;

    // let lookup: Vec<Vec<isize>> = input
    //     .iter()
    //     .cycle()
    //     .enumerate()
    //     .par_map(|(pos, _)| pattern_for_pos(pos, input_len))
    //     .collect();

    Vec::new()
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
        assert_eq!(pattern_for_pos(1, 3), vec![1, 0, -1]);
        assert_eq!(pattern_for_pos(2, 7), vec![0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(
            pattern_for_pos(3, 11),
            vec![0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]
        );

        let mut lookup = HashMap::new();
        assert_eq!(pattern(1, 3, &mut lookup).to_owned(), vec![1, 0, -1]);
        assert_eq!(
            pattern(2, 7, &mut lookup).to_owned(),
            vec![0, 1, 1, 0, 0, -1, -1]
        );
        assert_eq!(
            pattern(3, 11, &mut lookup).to_owned(),
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

    #[test]
    fn aoc16_run_2() {
        use super::*;

        assert_eq!(
            run_2("03036732577212944063491565474664", 100, 8),
            parse("84462026").unwrap().1
        );
        assert_eq!(
            run_2("02935109699940807407585447034323", 100, 8),
            parse("78725270").unwrap().1
        );
        assert_eq!(
            run_2("03081770884921959731165446850517", 100, 8),
            parse("53553731").unwrap().1
        );
    }
}
