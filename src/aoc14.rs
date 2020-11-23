use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use std::collections::HashMap;
use std::fs;

const FUEL: &str = "FUEL";
const ORE: &str = "ORE";

pub fn run() {
    let input = fs::read_to_string("day14.txt").unwrap();
    println!("14:1 {}", run_1(&input));
    println!("14:2 {}", run_2(&input));
}

fn run_1(input: &str) -> u64 {
    let (_, reactions) = parse(input).unwrap();

    let mut produced = HashMap::new();
    produce(1, FUEL, &reactions, &mut produced);

    *produced.get(ORE).unwrap()
}

fn run_2(input: &str) -> u64 {
    let (_, reactions) = parse(input).unwrap();

    let mut max_cur_fuel = 1;
    let max_ore = 1000000000000;

    let mut produced = HashMap::new();
    loop {
        produced.clear();
        max_cur_fuel *= 2;
        produce(max_cur_fuel, FUEL, &reactions, &mut produced);
        if produced.get(ORE).unwrap() > &max_ore {
            break;
        }
    }
    let mut min_cur_fuel = max_cur_fuel / 2;
    while (max_cur_fuel - min_cur_fuel) > 1 {
        produced.clear();
        let middle = (min_cur_fuel + max_cur_fuel) / 2;
        produce(middle, FUEL, &reactions, &mut produced);
        if produced.get(ORE).unwrap() > &max_ore {
            max_cur_fuel = middle;
        } else {
            min_cur_fuel = middle;
        }
    }
    min_cur_fuel
}
type Quantity<'a> = (u64, &'a str);

fn parse_chemical(i: &str) -> IResult<&str, Quantity> {
    separated_pair(
        nom::combinator::map(crate::helper::u32_val, |v| v as u64),
        space1,
        alpha1,
    )(i)
}

type Reaction<'a> = (Vec<Quantity<'a>>, Quantity<'a>);

#[derive(Debug, Clone)]
struct Output {
    output_quantity: u64,
    inputs: Vec<(String, u64)>,
}

fn parse_reaction(i: &str) -> IResult<&str, Reaction> {
    let (i, input) = separated_list1(tag(", "), parse_chemical)(i)?;

    // Get all input quantities;
    let mut quantities: Vec<u64> = input.iter().map(|q| q.0).collect();

    let (i, _) = tag(" => ")(i)?;
    let (i, output) = parse_chemical(i)?;

    // Add output quantity
    quantities.push(output.0);

    let input = input.iter().map(|i| (i.0, i.1)).collect();
    let output = (output.0, output.1);
    Ok((i, (input, output)))
}

type Reactions = HashMap<String, Output>;

fn parse(i: &str) -> IResult<&str, Reactions> {
    let (i, reactions) = separated_list1(newline, parse_reaction)(i)?;

    let mut h = HashMap::new();

    for (inputs, output) in reactions {
        assert!(!h.contains_key(&output.1.to_owned()));

        let o = Output {
            output_quantity: output.0,
            inputs: inputs
                .iter()
                .map(|(q, name)| (name.to_string(), *q))
                .collect(),
        };

        h.insert(output.1.to_string(), o);
    }

    Ok((i, h))
}
type Produced = HashMap<String, u64>;

fn produce(q: u64, name: &str, r: &Reactions, produced: &mut Produced) {
    let q = {
        let already_produced = produced.entry(name.to_string()).or_insert(0);

        // do we have extra material already produced?
        if q <= *already_produced {
            *already_produced -= q;
            return;
        }

        let q = q - *already_produced;
        *already_produced = 0;
        q
    };

    let reaction = r.get(name).unwrap();

    let num_reactions = (q as f64 / reaction.output_quantity as f64).ceil() as u64;

    for (name, q) in reaction.inputs.iter() {
        if name == ORE {
            *produced.entry(ORE.to_string()).or_insert(0) += q * num_reactions;
        } else {
            produce(q * num_reactions, name, r, produced);
        }
    }
    let already_produced = produced.entry(name.to_string()).or_insert(0);
    // dbg! {(&num_reactions,reaction.output_quantity, &q)};
    *already_produced += num_reactions * reaction.output_quantity - q;
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
        assert_eq!(input[1], (3, "BC"));
        assert_eq!(output, (1, "FUEL"));

        let (_, (input, output)) = parse_reaction("114 ORE => 4 BHXH").unwrap();
        assert_eq!(input.len(), 1);
        assert_eq!(input[0], (114, "ORE"));
        assert_eq!(output, (4, "BHXH"));

        let (_, (input, output)) = parse_reaction("10 ZJFM, 4 MVSHM => 8 LCDPV").unwrap();
        assert_eq!(input.len(), 2);
        assert_eq!(input[0], (10, "ZJFM"));
        assert_eq!(input[1], (4, "MVSHM"));
        assert_eq!(output, (8, "LCDPV"));
    }
    #[test]
    fn aoc14_produce() {
        //
        let (_, reactions) = super::parse(
            r#"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"#,
        )
        .unwrap();

        let mut produced = std::collections::HashMap::new();
        super::produce(30, "A", &reactions, &mut produced);
        assert_eq!(produced.get("ORE"), Some(&30));
        assert_eq!(produced.get("A"), Some(&0));

        let mut produced = std::collections::HashMap::new();
        super::produce(25, "A", &reactions, &mut produced);
        assert_eq!(produced.get("ORE"), Some(&30));
        assert_eq!(produced.get("A"), Some(&5));
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
        use super::*;

        assert_eq!(
            run_2(
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
            82892753
        );

        assert_eq!(
            run_2(
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
            5586022
        );

        assert_eq!(
            run_2(
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
            460664
        );
    }
}
