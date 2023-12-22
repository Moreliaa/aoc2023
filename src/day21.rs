use aoc_lib::map2d::Map2D;
use std::{collections::{HashMap, VecDeque}, thread::current};

pub fn run(input: String) {
    let mut map = Map2D::from_string(input.clone());
    println!("Day21 Pt1: {}", pt1(&mut map, 64));
    let mut map = Map2D::from_string(input.clone());
    println!("Day21 Pt2: {}", pt2(&mut map));
}


fn pt1(input: &mut Map2D<char>, target: i32) -> i32 {
    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut unseen: VecDeque<((i32, i32), i32)> = VecDeque::new();

    let mut x_start = 0;
    let mut y_start = 0;
    'x: for x in 0..input.width() {
        for y in 0..input.height() {
            if *input.get(x,y).unwrap() == 'S' {
                x_start = x;
                y_start = y;
                break 'x;
            }
        }
    }

    unseen.push_back(((x_start, y_start), 0));
    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut current_steps = 0;
    while unseen.len() > 0 {
        let current_node = unseen.pop_front().unwrap();
        if current_node.1 != current_steps {
            seen = HashMap::new();
            current_steps = current_node.1;
        }

        if seen.insert(current_node.0, current_node.1) != None {
            continue;
        }

        let next_steps = current_node.1 + 1;
        if next_steps > target {
            continue;
        }
        for i in 0..dir.len() {
            let d = dir[i];
            let next_coords = (current_node.0.0 + d.0, current_node.0.1 + d.1);
            if let Some(tile) = input.get(next_coords.0, next_coords.1) {
                if (*tile == '.' || *tile == 'S') && !seen.contains_key(&next_coords) {
                    unseen.push_back((next_coords, next_steps));
                }
            }
        }
    }
    /*println!("{:?}", seen.keys());
    for s in seen.keys() {
        input.set(s.0, s.1, 'O');
    }*/
    //input.print();
    seen.len() as i32
}

fn pt2(input: &mut Map2D<char>) -> i32 {
    let target = 26501365;
    let target =64;
    
    let mut seen: HashMap<(i32, i32), i32> = HashMap::new();
    let mut unseen: VecDeque<((i32, i32), i32)> = VecDeque::new();

    let mut x_start = 0;
    let mut y_start = 0;
    'x: for x in 0..input.width() {
        for y in 0..input.height() {
            if *input.get(x,y).unwrap() == 'S' {
                x_start = x;
                y_start = y;
                break 'x;
            }
        }
    }

    unseen.push_back(((x_start, y_start), 0));
    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut current_steps = 0;
    while unseen.len() > 0 {
        let current_node = unseen.pop_front().unwrap();
        if current_node.1 != current_steps {
            seen = HashMap::new();
            current_steps = current_node.1;
        }

        if seen.insert(current_node.0, current_node.1) != None {
            continue;
        }

        let next_steps = current_node.1 + 1;
        if next_steps > target {
            continue;
        }
        for i in 0..dir.len() {
            let d = dir[i];
            let mut next_coords = (current_node.0.0 + d.0, current_node.0.1 + d.1);
            if next_coords.0 < 0 {
                next_coords.0 += input.width();
            }
            if next_coords.0 >= input.width() {
                next_coords.0 -= input.width();
            }
            if next_coords.1 < 0 {
                next_coords.1 += input.height();
            }
            if next_coords.1 >= input.height() {
                next_coords.1 -= input.height();
            }


            if let Some(tile) = input.get(next_coords.0, next_coords.1) {
                if (*tile == '.' || *tile == 'S') && !seen.contains_key(&next_coords) {
                    unseen.push_back((next_coords, next_steps));
                }
            }
        }
    }
    println!("{:?}", seen.keys());
    for s in seen.keys() {
        if s.0 != x_start || s.1 != y_start{
        input.set(s.0, s.1, 'O');}
    }
    input.print();
    seen.len() as i32
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........".to_string();
        let mut map = Map2D::from_string(input);
        assert_eq!(pt1(&mut map, 6), 16);
    }
}
