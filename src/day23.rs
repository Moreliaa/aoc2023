use aoc_lib::map2d::Map2D;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let map = Map2D::from_string(input.clone());
    println!("Day23 Pt1: {}", pt1(&map));
    let mut map = Map2D::from_string(input);
    println!("Day23 Pt2: {}", pt2(&mut map));
}

#[derive(Clone)]
struct Path {
    seen: HashSet<(i32, i32)>,
    x: i32,
    y: i32,
    steps: i32,
    last_x: i32, // pt2
    last_y: i32,
    last_steps: i32,
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
        last_x: 0,
        last_y: 0,
        last_steps: 0,
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

fn pt2(map: &mut Map2D<char>) -> i32 {
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

    let mut paths = vec![Path {
        seen: HashSet::new(),
        x: start_x,
        y: start_y,
        steps: 0,
        last_x: start_x,
        last_y: start_y,
        last_steps: 0,
    }];
    let mut graph: HashMap<(i32, i32), HashMap<(i32, i32), i32>> = HashMap::new();
    graph.insert((start_x, start_y), HashMap::new());
    let mut seen_graph: HashSet<(i32, i32)> = HashSet::new();

    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    while let Some(mut path) = paths.pop() {
        if path.x == end_x && path.y == end_y {
            if !graph.contains_key(&(end_x, end_y)) {
                graph.insert((end_x, end_y), HashMap::new());
            }
            let dist = path.steps - path.last_steps;
            graph.entry((end_x, end_y)).and_modify(|e| {
                e.insert((path.last_x, path.last_y), dist);
            });
            graph.entry((path.last_x, path.last_y)).and_modify(|e| {
                e.insert((end_x, end_y), dist);
            });
            continue;
        }
        let mut possible_steps = vec![];
        for d in dir.iter() {
            let x1 = path.x + d.0;
            let y1 = path.y + d.1;
            if path.seen.contains(&(x1, y1)) {
                continue;
            }

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
        /*if (path.x != start_x || path.y != start_y) && graph.contains_key(&(path.x, path.y)) {
            continue;
        }*/

        if poss_len > 1 {
            if !graph.contains_key(&(path.x, path.y)) {
                graph.insert((path.x, path.y), HashMap::new());
            }

            let dist = path.steps - path.last_steps;
            graph.entry((path.x, path.y)).and_modify(|e| {
                e.insert((path.last_x, path.last_y), dist);
            });
            graph.entry((path.last_x, path.last_y)).and_modify(|e| {
                e.insert((path.x, path.y), dist);
            });
        }

        if seen_graph.contains(&(path.x, path.y)) {
            continue;
        }
        if poss_len > 1 {
            seen_graph.insert((path.x, path.y));
        }

        for (idx, (x1, y1)) in possible_steps.into_iter().enumerate() {
            /*if idx == poss_len - 1 {
                path.seen.insert((path.x, path.y));
                if poss_len > 1 {
                    path.last_x = path.x;
                    path.last_y = path.y;
                    path.last_steps = path.steps;
                }
                path.x = x1;
                path.y = y1;
                path.steps += 1;
                paths.push(path);
                break;
            //} else {*/
            let mut new_path = path.clone();
            new_path.seen.insert((path.x, path.y));
            if poss_len > 1 {
                new_path.last_x = path.x;
                new_path.last_y = path.y;
                new_path.last_steps = path.steps;
            }
            new_path.x = x1;
            new_path.y = y1;
            new_path.steps += 1;
            paths.push(new_path);
            //}
        }
    }

    // find paths
    let mut steps: Vec<i32> = vec![];
    let mut paths = vec![Path {
        seen: HashSet::new(),
        x: start_x,
        y: start_y,
        steps: 0,
        last_x: 0,
        last_y: 0,
        last_steps: 0,
    }];

    for coords in graph.keys() {
        map.set(coords.0, coords.1, 'O');
    }
    map.print();

    while let Some(path) = paths.pop() {
        if path.x == end_x && path.y == end_y {
            steps.push(path.steps);
            continue;
        }
        // possible steps
        let info = graph.get(&(path.x, path.y)).unwrap();
        for (coords, steps) in info.iter() {
            if path.seen.contains(&(coords.0, coords.1)) {
                continue;
            }
            let mut new_path = path.clone();
            new_path.seen.insert((path.x, path.y));
            new_path.x = coords.0;
            new_path.y = coords.1;
            new_path.steps += steps;
            paths.push(new_path);
        }
    }
    steps.into_iter().fold(0, |acc, v| acc.max(v))
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
        let mut map = Map2D::from_string(input);
        assert_eq!(pt1(&map), 94);
        assert_eq!(pt2(&mut map), 154);
    }
}
