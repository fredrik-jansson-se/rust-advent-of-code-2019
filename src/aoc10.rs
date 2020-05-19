use std::collections::{HashMap, HashSet};
use std::fs;

pub fn run() {
    let input = fs::read_to_string("day10.txt").unwrap();
    let (_, m) = run_1(&input);
    println!("10:1 {:?}", m);
}

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else if a < b {
        gcd(b, a)
    } else {
        gcd(b, a % b)
    }
}

// Calculate line equation on the form
// a*y = b*x + m
// This allows us to handle vertical lines where a=0
fn line_eq((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> (i32, i32, i32) {
    let a = x2 - x1;
    let b = y2 - y1;
    let g = gcd(a.abs(), b.abs());
    let a = a / g;
    let b = b / g;
    let m = a * y1 - b * x1;
    (a, b, m)
}

fn run_1(input: &str) -> ((usize, usize), usize) {
    let map = parse(input);

    let mut visible = HashMap::new();

    for (source_y, source_row) in map.iter().enumerate() {
        for (source_x, s) in source_row.iter().enumerate() {
            if !s {
                continue;
            }

            let mut lines_checked = HashSet::new();

            // Look at all asteroids that are at higher x and y.
            for (dest_y, dest_row) in map.iter().enumerate().skip(source_y) {
                // If on the same row (y) only consider asteroids to the right,
                // otherwise all asteroids on the next rows
                let to_skip = if dest_y == source_y { source_x + 1 } else { 0 };
                for (dest_x, d) in dest_row.iter().enumerate().skip(to_skip) {
                    if !d {
                        continue;
                    }
                    let l_eq = line_eq(
                        (source_x as i32, source_y as i32),
                        (dest_x as i32, dest_y as i32),
                    );

                    // If we have previously followed this trajectory, we know there's an asteroid
                    // between blocking view of this
                    if lines_checked.contains(&l_eq) {
                        continue;
                    }

                    lines_checked.insert(l_eq);

                    // Update count for source
                    let asteroid_cnt = visible.entry((source_x, source_y)).or_insert(0);
                    *asteroid_cnt += 1;

                    // Update count for dest
                    let asteroid_cnt = visible.entry((dest_x, dest_y)).or_insert(0);
                    *asteroid_cnt += 1;
                    // println!(
                    //     "({}, {}) -> ({}, {}), {:?}",
                    //     source_x, source_y, dest_x, dest_y, l_eq
                    // );
                }
            }
        }
    }

    let (max_ast, m) = visible
        .iter()
        .max_by(|(_, cnt1), (_, cnt2)| cnt1.cmp(cnt2))
        .unwrap();
    (*max_ast, *m)
}

fn parse(i: &str) -> Vec<Vec<bool>> {
    let mut map = Vec::new();
    for line in i.lines() {
        let row = line.chars().map(|c| c == '#').collect();

        map.push(row);
    }
    map
}

fn run_2(input: &str, (x, y): (usize, usize)) -> usize {
    0
}

#[cfg(test)]
mod tests {
    #[test]
    fn aoc10_parse() {
        use super::*;
        let map = parse(
            r#".#..#
.....
#####
....#
...##"#,
        );
        assert_eq!(map.len(), 5);
        assert_eq!(map[0].len(), 5);
        assert!(map[2][0]);
        assert!(!map[3][0]);
    }

    #[test]
    fn aoc10_run_1() {
        use super::*;

        assert_eq!(
            run_1(
                r#"#..
.#.
..#"#
            ),
            ((1, 1), 2)
        );

        assert_eq!(
            run_1(
                r#".#..#
.....
#####
....#
...##"#
            ),
            ((3, 4), 8)
        );

        assert_eq!(
            run_1(
                r#"......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####"#
            ),
            ((5, 8), 33)
        );

        assert_eq!(
            run_1(
                r#"#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###."#
            ),
            ((1, 2), 35)
        );

        assert_eq!(
            run_1(
                r#".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#.."#
            ),
            ((6, 3), 41)
        );

        assert_eq!(
            run_1(
                r#".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##"#
            ),
            ((11, 13), 210)
        );
    }

    #[test]
    fn aoc10_run_2() {
        use super::*;

        assert_eq!(
            run_2(
                r#".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##"#,
                (11, 13)
            ),
            802
        );
    }
}
