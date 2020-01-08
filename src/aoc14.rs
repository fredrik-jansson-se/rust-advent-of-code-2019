use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    combinator::map,
    multi::separated_nonempty_list,
    sequence::separated_pair,
    IResult,
};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day14.txt").unwrap();
    println!("14:1 {}", run_1(&input));
    // println!("13:2 {}", run_2(&input));
}

fn ingredients_to(target: &str, reactions: &[Reaction], current: &Quantity) -> u32 {
    if target == current.1 {
        current.0
    } else {
        // dbg! {&reactions};
        // dbg! {current};
        let reaction: &Reaction = reactions
            .iter()
            .find(|(_, (_, name))| *name == current.1)
            .unwrap();
        dbg! {reaction};

        let ingredients: u32 = reaction
            .0
            .iter()
            .map(|r| ingredients_to(target, reactions, r))
            .sum();
        current.0 * ingredients
    }
}

fn run_1(input: &str) -> u32 {
    let (_, reactions) = parse(input).unwrap();

    let fuel_reaction: &Reaction = reactions
        .iter()
        .find(|(_, (_, name))| *name == "FUEL")
        .unwrap();

    ingredients_to("ORE", &reactions, &fuel_reaction.1)
}

type Quantity<'a> = (u32, &'a str);

fn parse_chemical(i: &str) -> IResult<&str, Quantity> {
    separated_pair(crate::helper::u32_val, space1, alpha1)(i)
}

type Reaction<'a> = (Vec<Quantity<'a>>, Quantity<'a>);

fn parse_reaction(i: &str) -> IResult<&str, Reaction> {
    let (i, input) = separated_nonempty_list(tag(", "), parse_chemical)(i)?;
    let (i, _) = tag(" => ")(i)?;
    let (i, output) = parse_chemical(i)?;
    Ok((i, (input, output)))
}

fn parse(i: &str) -> IResult<&str, Vec<Reaction>> {
    separated_nonempty_list(newline, parse_reaction)(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc14_parse() {
        use super::*;
        assert_eq!(parse_chemical("144 ORE"), Ok(("", (144, "ORE"))));

        let (_, (input, output)) = parse_reaction("2 AB, 3 BC, 4 CA => 1 FUEL").unwrap();
        assert_eq!(input.len(), 3);
        assert_eq!(input[0], (2, "AB"));
        assert_eq!(output, (1, "FUEL"));
    }

    #[test]
    fn aoc14_ingredients() {
        use super::*;
        let (_, (input, output)) = parse_reaction("2 AB, 3 BC, 4 CA => 1 FUEL").unwrap();

        assert_eq!(
            ingredients_to("AB", &vec![(input, output)], &(1, "FUEL")),
            2
        );
    }

    #[test]
    fn aoc14_run_1() {
        use super::*;
        assert_eq!(
            run_1(
                r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#
            ),
            31
        );

        assert_eq!(
            run_1(
                r#"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"#
            ),
            165
        );

        assert_eq!(
            run_1(
                r#"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"#
            ),
            13312
        );

        assert_eq!(
            run_1(
                r#"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF"#
            ),
            180697
        );

        assert_eq!(
            run_1(
                r#"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX"#
            ),
            2210736
        );
    }

    #[test]
    fn aoc14_run_2() {
        // assert_eq!(9, run_2(51589));
        // assert_eq!(6, run_2(01245));
        // assert_eq!(18, run_2(92510));
        // assert_eq!(2018, run_2(59414));
    }
}
