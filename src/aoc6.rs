use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, newline};
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day6.txt").unwrap();
    println!("6:1: {}", run_1(&input));
    println!("6:2: {}", run_2(&input));
}

#[derive(Debug)]
struct Tree {
    name: String,
    children: Vec<Tree>,
    orbits: usize,
}

impl Tree {
    fn new(name: &str) -> Self {
        Tree {
            name: name.to_owned(),
            children: Vec::new(),
            orbits: 0,
        }
    }
}

fn _build_tree(node: &mut Tree, lookup: &HashMap<String, Vec<String>>) {
    let empty = Vec::new();
    let children = lookup.get(&node.name).unwrap_or(&empty);
    for c in children {
        let mut child = Tree::new(c);
        _build_tree(&mut child, lookup);
        node.children.push(child);
    }
}

fn build_tree(pairs: &[(&str, &str)]) -> Tree {
    let mut tree = HashMap::new();

    let mut potential_roots = HashSet::new();
    let mut inner = HashSet::new();

    for (o, i) in pairs {
        inner.insert(i);
        potential_roots.remove(i);
        if !inner.contains(o) {
            potential_roots.insert(o);
        }

        let outer = tree.entry((*o).to_owned()).or_insert(Vec::new());
        outer.push((*i).to_owned());
    }

    assert_eq!(1, potential_roots.len());

    let root = potential_roots.iter().next().unwrap();

    let mut the_root = Tree::new(root);

    _build_tree(&mut the_root, &tree);

    the_root
}

fn calc_orbits(node: &mut Tree, parent: Option<usize>) {
    node.orbits = parent.map_or(0, |p| p + 1);
    let o = Some(node.orbits);
    node.children.iter_mut().for_each(|c| calc_orbits(c, o));
}

fn sum_orbits(node: &Tree) -> usize {
    let c_sum: usize = node.children.iter().map(sum_orbits).sum();
    node.orbits + c_sum
}

fn run_1(input: &str) -> usize {
    let (_, pairs) = parse(input).unwrap();

    let mut tree = build_tree(&pairs);

    calc_orbits(&mut tree, None);

    sum_orbits(&tree)
}

fn build_path_to(node: &Tree, to: &str) -> Option<Vec<String>> {
    if node.name == to {
        Some(Vec::new())
    } else {
        match node
            .children
            .iter()
            .filter_map(|c| build_path_to(c, to))
            .next()
        {
            None => None,
            Some(mut v) => {
                v.push(node.name.clone());
                Some(v)
            }
        }
    }
}

fn run_2(input: &str) -> usize {
    let (_, pairs) = parse(input).unwrap();

    let tree = build_tree(&pairs);

    let mut you_path = build_path_to(&tree, "YOU").unwrap();
    let mut san_path = build_path_to(&tree, "SAN").unwrap();

    you_path.reverse();
    san_path.reverse();

    let common_len = you_path
        .iter()
        .zip(san_path.iter())
        .take_while(|(a, b)| a == b)
        .count();

    // dbg! { (&you_path, &san_path, common_len)};

    you_path.len() + san_path.len() - 2 * common_len
}

fn parse(i: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        newline,
        separated_pair(alphanumeric1, tag(")"), alphanumeric1),
    )(i)
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc6_parse() {
        use super::*;
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        let (_, pairs) = dbg! {parse(input).unwrap()};
        assert_eq!(pairs.len(), 11);
    }

    #[test]
    fn aoc6_run_1() {
        use super::*;
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L"#;
        assert_eq!(run_1(input), 42);
    }

    #[test]
    fn aoc6_run_2() {
        use super::*;
        let input = r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#;
        assert_eq!(run_2(input), 4);
    }
}
