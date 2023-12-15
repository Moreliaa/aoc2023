use aoc_lib::map2d::Map2D;
use std::collections::HashSet;

pub fn run(input: String) {
    {
        let mut input = Map2D::from_string(input.clone());
        println!("Day14 Pt1: {}", pt1(&mut input));
    }

    let mut input = Map2D::from_string(input);
    println!("Day14 Pt2: {}", pt2(&mut input));
}

fn pt1(input: &mut Map2D<char>) -> i32 {
    tilt(input, 'N');
    get_load(input)
}

fn tilt(input: &mut Map2D<char>, dir: char) {
    match dir {
        'N' => {
            for y in 0..input.height() {
                for x in 0..input.width() {
                    let tile = *input.get(x, y).unwrap();
                    if tile == 'O' {
                        input.set(x, y, '.');
                        let offset = -1;
                        let mut y_roll = y + offset;
                        loop {
                            match input.get(x, y_roll) {
                                Some(val) => match val {
                                    'O' | '#' => {
                                        input.set(x, y_roll - offset, 'O');
                                        break;
                                    }
                                    _ => {
                                        y_roll += offset;
                                    }
                                },
                                None => {
                                    input.set(x, y_roll - offset, 'O');
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        'S' => {
            for y in (0..input.height()).rev() {
                for x in 0..input.width() {
                    let tile = *input.get(x, y).unwrap();
                    if tile == 'O' {
                        input.set(x, y, '.');
                        let offset = 1;
                        let mut y_roll = y + offset;
                        loop {
                            match input.get(x, y_roll) {
                                Some(val) => match val {
                                    'O' | '#' => {
                                        input.set(x, y_roll - offset, 'O');
                                        break;
                                    }
                                    _ => {
                                        y_roll += offset;
                                    }
                                },
                                None => {
                                    input.set(x, y_roll - offset, 'O');
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        'W' => {
            for x in 0..input.width() {
                for y in 0..input.height() {
                    let tile = *input.get(x, y).unwrap();
                    if tile == 'O' {
                        input.set(x, y, '.');
                        let offset = -1;
                        let mut x_roll = x + offset;
                        loop {
                            match input.get(x_roll, y) {
                                Some(val) => match val {
                                    'O' | '#' => {
                                        input.set(x_roll - offset, y, 'O');
                                        break;
                                    }
                                    _ => {
                                        x_roll += offset;
                                    }
                                },
                                None => {
                                    input.set(x_roll - offset, y, 'O');
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        'E' => {
            for x in (0..input.width()).rev() {
                for y in 0..input.height() {
                    let tile = *input.get(x, y).unwrap();
                    if tile == 'O' {
                        input.set(x, y, '.');
                        let offset = 1;
                        let mut x_roll = x + offset;
                        loop {
                            match input.get(x_roll, y) {
                                Some(val) => match val {
                                    'O' | '#' => {
                                        input.set(x_roll - offset, y, 'O');
                                        break;
                                    }
                                    _ => {
                                        x_roll += offset;
                                    }
                                },
                                None => {
                                    input.set(x_roll - offset, y, 'O');
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => panic!(),
    }
}

fn get_load(input: &Map2D<char>) -> i32 {
    input.aggregate(|c, _, y| if *c != 'O' { 0 } else { input.height() - y })
}

fn pt2(input: &mut Map2D<char>) -> i32 {
    let mut loads: HashSet<i32> = HashSet::new();
    let mut loads_vec: Vec<i32> = vec![];
    let mut cycles = 0;
    let min_cycles = 200;
    loop {
        cycles += 1;
        for d in ['N', 'W', 'S', 'E'] {
            tilt(input, d);
        }
        let load = get_load(input);
        loads_vec.push(load);
        if loads.contains(&load) && cycles > min_cycles {
            break;
        } else {
            loads.insert(load);
        }
    }

    let target = 1000000000;
    let last = loads_vec.last().unwrap();
    let mut cycle_len = 0;
    for i in (0..loads_vec.len() - 1).rev() {
        if loads_vec[i] != *last {
            continue;
        }

        cycle_len = loads_vec.len() - i - 1;
        break;
    }

    let slice = &loads_vec[loads_vec.len() - cycle_len..loads_vec.len()];
    let mut idx_first_cycle = 0;
    'outer: for i in 0..loads_vec.len() {
        for j in i..i + cycle_len {
            if slice[j - i] != loads_vec[j] {
                continue 'outer;
            }
        }
        idx_first_cycle = i;
        break;
    }
    let mut offset = (target - idx_first_cycle - 1) % cycle_len;
    offset += idx_first_cycle;
    return loads_vec[offset];
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = Map2D::from_string(
            "OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#...."
                .to_string(),
        );
        assert_eq!(get_load(&input), 136);
    }
}
