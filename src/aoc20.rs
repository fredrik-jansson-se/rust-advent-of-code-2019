use std::fs;

pub fn run() {
    let input = fs::read_to_string("day20.txt").unwrap();
    println!("20:1: {}", run_1(&input));
    println!("20:2: {}", run_2(&input));
}

#[derive(Debug, PartialEq)]
enum Type {
    Passage,
    Wall,
    Portal(String),
}

fn parse(input: &str) -> Vec<Vec<Type>> {
    let mut map = Vec::new();
    let lns = input.lines();
    for (_row_no, row) in lns.enumerate() {
        let mut map_row = Vec::new();
        for p in row.chars() {
            match p {
                '.' => {
                    // Do we have portal names north of?
                    map_row.push(Type::Passage);
                }
                _ => map_row.push(Type::Wall),
            }
        }
        map.push(map_row)
    }
    map
}

fn run_1(input: &str) -> i64 {
    let _map = parse(input);
    0
}

fn run_2(_input: &str) -> i64 {
    unreachable!();
}

#[cfg(test)]
mod tests {

    #[test]
    fn aoc20_run_1() {
        use super::*;
        let input = r#"         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       "#;
        let map = parse(input);
        assert_eq!(map[2][9], Type::Portal("AA".to_owned()));
    }
}
