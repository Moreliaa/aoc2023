use aoc_lib::map2d::Map2D;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let map = Map2D::from_string(input);
    println!("Day23 Pt1: {}", pt1(&map));
    //println!("Day23 Pt2: {}", pt2(&input));
}

#[derive(Clone)]
struct Path {
    seen: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    steps: i32,
}

fn pt1(map: &Map2D<char>) -> i32 {
    let (mut start_x, start_y) = (0, 0);
    for x in 0..map.width() {
        if *map.get(x, start_y).unwrap() != '#' {
            start_x = x;
            break;
        }
    }

    let (mut end_x, end_y) = (0, map.height() - 1);
    for x in 0..map.width() {
        if *map.get(x, end_y).unwrap() != '#' {
            end_x = x;
            break;
        }
    }

    let mut steps: Vec<i32> = vec![];
    let mut paths = vec![Path {
        seen: HashSet::new(),
        x: start_x,
        y: start_y,
        steps: 0,
    }];

    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let coord = ['N', 'S', 'W', 'E'];
    while let Some(mut path) = paths.pop() {
        if path.x == end_x && path.y == end_y {
            steps.push(path.steps);
            continue;
        }
        let mut possible_steps = vec![];
        for (idx, d) in dir.iter().enumerate() {
            let x1 = path.x + d.0;
            let y1 = path.y + d.1;
            if path.seen.contains(&(x1, y1)) {
                continue;
            }
            match map.get(path.x, path.y).unwrap() {
                '<' => {
                    if coord[idx] != 'W' {
                        continue;
                    }
                }
                '^' => {
                    if coord[idx] != 'N' {
                        continue;
                    }
                }
                'v' => {
                    if coord[idx] != 'S' {
                        continue;
                    }
                }
                '>' => {
                    if coord[idx] != 'E' {
                        continue;
                    }
                }
                _ => (),
            };

            if let Some(tile) = map.get(x1, y1) {
                match *tile {
                    '#' => continue,
                    _ => (),
                }
            } else {
                continue;
            }

            possible_steps.push((x1, y1));
        }

        let poss_len = possible_steps.len();
        for (idx, (x1, y1)) in possible_steps.into_iter().enumerate() {
            if idx == poss_len - 1 {
                path.seen.insert((path.x, path.y));
                path.x = x1;
                path.y = y1;
                path.steps += 1;
                paths.push(path);
                break;
            } else {
                let mut new_path = path.clone();
                new_path.seen.insert((path.x, path.y));
                new_path.x = x1;
                new_path.y = y1;
                new_path.steps += 1;
                paths.push(new_path);
            }
        }
    }

    println!("{:?}", steps);
    steps.into_iter().fold(0, |acc, v| acc.max(v))
}

fn pt2(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#"
            .to_string();
        let map = Map2D::from_string(input);
        assert_eq!(pt1(&map), 94);
    }
}
