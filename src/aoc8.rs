use nom::{bytes::complete::take_while1, character::is_digit, combinator::map, IResult};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day8.txt").unwrap();
    // 1848 too low
    println!("day8-1: {}", run_1(&input, 25, 6));
    println!("day8-2:\n{}", run_2(&input, 25, 6));
}

fn is_char_digit(chr: char) -> bool {
    chr.is_ascii() && is_digit(chr as u8)
}

fn parse(i: &str) -> IResult<&str, Vec<u32>> {
    map(take_while1(is_char_digit), |digits: &str| {
        digits.chars().map(|c| c.to_digit(10).unwrap()).collect()
    })(i)
}

fn run_1(input: &str, width: usize, height: usize) -> u32 {
    let (_, image) = parse(input).unwrap();

    let min_zero_layer = image
        .chunks(width * height)
        .min_by_key(|layer| layer.iter().filter(|x| **x == 0).count())
        .unwrap();
    let (ones, twos) = min_zero_layer.iter().fold((0, 0), |(one_sum, two_sum), d| {
        (
            one_sum + if *d == 1 { 1 } else { 0 },
            two_sum + if *d == 2 { 1 } else { 0 },
        )
    });
    ones * twos
}

fn run_2(input: &str, width: usize, height: usize) -> String {
    let (_, image) = parse(input).unwrap();
    let layers = image.chunks(width * height).collect::<Vec<_>>();

    let mut res = String::new();

    for row in 0..height {
        for col in 0..width {
            for l in layers.iter() {
                let color = l[row * width + col];
                if color == 0 {
                    res += " ";
                    break;
                } else if color == 1 {
                    res += "*";
                    break;
                }
            }
        }
        res += "\n";
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aoc8_run_1() {
        assert_eq!(run_1("123456789012", 3, 2), 1);
    }

    #[test]
    fn aoc8_run_2() {
        assert_eq!(run_2("0222112222120000", 2, 2), " *\n* \n");
    }
}
