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

    let mut first = true;
    while res.len() < input_len {
        for v in BASE_PATTERN.iter() {
            for _ in 0..pos {
                if first {
                    first = false;
                } else {
                    res.push(*v);
                }
            }
        }
    }

    // res.into_iter().skip(1).take(input_len).collect()
    res.resize(input_len, 0);
    res
}

fn pattern(pos: usize, input_len: usize, lookup: &mut HashMap<usize, Vec<isize>>) -> &[isize] {
    lookup.entry(pos).or_insert(pattern_for_pos(pos, input_len))
}

fn get_index(data: &[isize]) -> usize {
    let mut idx = 0;

    for i in data.iter().take(7) {
        idx = (idx * 10) + *i as usize;
    }

    idx
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

fn run_2(input: &str, _iterations: usize, digits: usize) -> Vec<isize> {
    let (_, mut input) = parse(input).unwrap();

    let message_offset = get_index(&input);

    {
        let orig = input.clone();
        for _ in 0..10000 {
            input.extend(&orig);
        }
    }

    let mut lookup = Vec::with_capacity(input.len());
    (0..input.len())
        .into_par_iter()
        .map(|i| pattern_for_pos(i + 1, input.len()))
        .collect_into_vec(&mut lookup);
    // for i in 0..input.len() {
    //     lookup.push(pattern_for_pos(i + 1, lookup.len()));
    // }
    // let lookup: Vec<Vec<isize>> = input
    //     .iter()
    //     .cycle()
    //     .enumerate()
    //     .par_map(|(pos, _)| pattern_for_pos(pos, input_len))
    //     .collect();

    input
        .iter()
        .skip(message_offset)
        .take(digits)
        .map(|i| *i)
        .collect()
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
    fn aoc16_run_get_index() {
        use super::*;
        let (_, input) = parse("03036732577212944063491565474664").unwrap();
        assert_eq!(get_index(&input), 0303673);

        let (_, input) = parse("02935109699940807407585447034323").unwrap();
        assert_eq!(get_index(&input), 0293510);

        let (_, input) = parse("03081770884921959731165446850517").unwrap();
        assert_eq!(get_index(&input), 0308177);
    }

    #[test]
    fn aoc16_run_2() {
        assert_eq!(
            super::run_2("03036732577212944063491565474664", 100, 8),
            super::parse("84462026").unwrap().1
        );
        assert_eq!(
            super::run_2("02935109699940807407585447034323", 100, 8),
            super::parse("78725270").unwrap().1
        );
        assert_eq!(
            super::run_2("03081770884921959731165446850517", 100, 8),
            super::parse("53553731").unwrap().1
        );
    }
}
