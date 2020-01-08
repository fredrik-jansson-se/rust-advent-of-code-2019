use super::helper::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{newline, space0};
use nom::combinator::{map, opt};
use nom::multi::separated_nonempty_list;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day24.txt").unwrap();
    println!("24:1 {}", run_1(&input));
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum AttackType {
    Radiation,
    Bludgenoning,
    Fire,
    Slashing,
    Cold,
}

impl AttackType {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            preceded(
                space0,
                alt((
                    tag("bludgeoning"),
                    tag("radiation"),
                    tag("fire"),
                    tag("slashing"),
                    tag("cold"),
                )),
            ),
            |s| match s {
                "radiation" => AttackType::Radiation,
                "bludgeoning" => AttackType::Bludgenoning,
                "fire" => AttackType::Fire,
                "slashing" => AttackType::Slashing,
                "cold" => AttackType::Cold,
                fail => {
                    dbg! {fail};
                    unreachable!()
                }
            },
        )(i)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Attribute {
    Weak(Vec<AttackType>),
    Immune(Vec<AttackType>),
}

impl Attribute {
    fn parse(i: &str) -> IResult<&str, Vec<Self>> {
        preceded(
            tag("("),
            terminated(
                separated_nonempty_list(
                    tag("; "),
                    alt((Attribute::parse_weak, Attribute::parse_immune)),
                ),
                tag(")"),
            ),
        )(i)
    }

    fn parse_immune(i: &str) -> IResult<&str, Attribute> {
        map(
            preceded(
                tag("immune to "),
                separated_nonempty_list(tag(","), AttackType::parse),
            ),
            Attribute::Immune,
        )(i)
    }

    fn parse_weak(i: &str) -> IResult<&str, Attribute> {
        map(
            preceded(
                tag("weak to "),
                separated_nonempty_list(tag(","), AttackType::parse),
            ),
            Attribute::Weak,
        )(i)
    }
}

#[derive(Debug, PartialEq)]
struct Group {
    army_name: String,
    units: usize,
    hit_points: usize,
    weakness: Vec<AttackType>,
    immune: Vec<AttackType>,
    attack: usize,
    attack_type: AttackType,
    initiative: usize,
}

impl Group {
    fn _attr_to_iw(attrs: &[Attribute]) -> (Vec<AttackType>, Vec<AttackType>) {
        let empty = Vec::new();
        let w = attrs
            .iter()
            .find_map(|a| match a {
                Attribute::Weak(a) => Some(a),
                _ => None,
            })
            .unwrap_or(&empty);

        let i = attrs
            .iter()
            .find_map(|a| match a {
                Attribute::Immune(a) => Some(a),
                _ => None,
            })
            .unwrap_or(&empty);

        (w.to_vec(), i.to_vec())
    }

    fn parse(army_name: String) -> impl Fn(&str) -> IResult<&str, Self> {
        move |i| {
            let (i, units) = u32_val(i)?;
            let (i, _) = tag(" units each with ")(i)?;
            let (i, hit_points) = u32_val(i)?;
            let (i, _) = tag(" hit points ")(i)?;
            let (i, attrs) = opt(Attribute::parse)(i)?;
            let (weakness, immune) =
                attrs.map_or(([].to_vec(), [].to_vec()), |a| Self::_attr_to_iw(&a));
            let (i, _) = space0(i)?;
            let (i, _) = tag("with an attack that does ")(i)?;
            let (i, attack) = u32_val(i)?;
            let (i, _) = space0(i)?;
            let (i, attack_type) = AttackType::parse(i)?;
            let (i, _) = tag(" damage at initiative ")(i)?;
            let (i, initiative) = u32_val(i)?;
            Ok((
                i,
                Self {
                    army_name: army_name.clone(),
                    units: units as usize,
                    hit_points: hit_points as usize,
                    weakness: weakness,
                    immune: immune,
                    attack: attack as usize,
                    attack_type: attack_type,
                    initiative: initiative as usize,
                },
            ))
        }
    }
}

fn parse_army(i: &str) -> IResult<&str, Vec<Group>> {
    let (i, name) = take_until(":")(i)?;
    let (i, _) = tag(":")(i)?;
    let (i, _) = newline(i)?;
    let (i, groups) = separated_nonempty_list(newline, Group::parse(name.to_owned()))(i)?;

    Ok((i, groups))
}

fn parse_armies(i: &str) -> IResult<&str, (Vec<Group>, Vec<Group>)> {
    let (i, immune) = parse_army(i)?;
    let (i, _) = newline(i)?;
    let (i, _) = newline(i)?;
    let (i, inf) = parse_army(i)?;
    Ok((i, (immune, inf)))
}

fn attack(g: &mut [Group], a: usize, d: usize) {
    println!("{} attacks {}", &g[a].army_name, &g[d].army_name);
    let dmg_multiplier = if g[d].weakness.contains(&g[a].attack_type) {
        2
    } else if g[d].immune.contains(&g[a].attack_type) {
        0
    } else {
        1
    };

    let dmg = g[a].attack * dmg_multiplier;

    let total_dmg = (dmg * g[a].units) as isize;
    let total_def = (g[d].units * g[d].hit_points) as isize;
    let alive = isize::max(total_def - total_dmg, 0) as usize / g[d].hit_points;
    println!("{} damage to {} hp, {} alive", total_dmg, total_def, alive);
    g[d].units = alive;
}

fn run_1(input: &str) -> usize {
    let (_, (mut a1, mut a2)) = parse_armies(input).unwrap();
    a1.append(&mut a2);
    loop {
        a1.sort_by(|g1, g2| g1.initiative.cmp(&g2.initiative));
        let mut to_remove = Vec::new();
        for i in 0..a1.len() {
            for j in (i + 1)..a1.len() {
                if a1[i].army_name != a1[j].army_name {
                    attack(&mut a1, i, j);

                    let d = &a1[j];
                    if d.units == 0 {
                        to_remove.push(j);
                    }
                }
            }
        }

        to_remove.sort();
        to_remove.reverse();

        for idx in to_remove {
            a1.remove(idx);
        }

        let first_name = &a1[0].army_name;
        if a1.iter().all(|g| &g.army_name == first_name) {
            break;
        }
    }
    a1.iter().map(|g| g.units).sum()
}

mod tests {
    use super::*;

    #[test]
    fn aoc24_parse_group() {
        assert_eq!(
            Ok(("", AttackType::Bludgenoning)),
            AttackType::parse("bludgeoning")
        );
        assert_eq!(
            Ok(("", AttackType::Radiation)),
            AttackType::parse("radiation")
        );

        assert_eq!(
            Ok(("", Attribute::Immune([AttackType::Fire].to_vec()))),
            Attribute::parse_immune("immune to fire")
        );

        assert_eq!(
            Ok((
                "",
                Attribute::Weak([AttackType::Radiation, AttackType::Bludgenoning].to_vec())
            )),
            Attribute::parse_weak("weak to radiation, bludgeoning")
        );

        assert_eq!(
            Ok((
                "",
                [
                    Attribute::Immune([AttackType::Fire].to_vec()),
                    Attribute::Weak([AttackType::Bludgenoning, AttackType::Slashing].to_vec())
                ]
                .to_vec()
            )),
            Attribute::parse("(immune to fire; weak to bludgeoning, slashing)")
        );
        assert_eq!(
            Ok((
                " ",
                [Attribute::Weak(
                    [AttackType::Radiation, AttackType::Bludgenoning].to_vec()
                ),]
                .to_vec()
            )),
            Attribute::parse("(weak to radiation, bludgeoning) ")
        );

        assert_eq!(
            Ok((
                "",
                Group {
                    army_name:"The Army".to_owned(),
                    units: 17,
                    hit_points: 5390,
                    weakness: [AttackType::Radiation, AttackType::Bludgenoning].to_vec(),
                    immune: [].to_vec(),
                    attack: 4507,
                    attack_type: AttackType::Fire,
                    initiative: 2
                }
            )),
            Group::parse("The Army".to_owned())("17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2")
        );
        assert_eq!(
            Ok((
                "\n",
                Group {
                    army_name:"The Army".to_owned(),
                    units: 4154,
                    hit_points: 3839,
                    weakness: [].to_vec(),
                    immune: [].to_vec(),
                    attack:9,
                    attack_type: AttackType::Slashing,
                    initiative: 7
                }
            )),
            Group::parse("The Army".to_owned())("4154 units each with 3839 hit points with an attack that does 9 slashing damage at initiative 7\n")
        );

        let armies = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

        let (_, (imm, inf)) = parse_armies(armies).unwrap();
        assert_eq!("Immune System", imm[0].army_name);
        assert_eq!(2, imm.len());
        assert_eq!("Infection", inf[0].army_name);
        assert_eq!(2, inf.len());
    }

    #[test]
    fn aoc24_1() {
        let armies = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(5216, run_1(armies));
    }

    #[test]
    fn aoc24_2() {}
}
