use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    let input = parse_input(input);
    println!("Day13 Pt1: {}", pt1(&input));
    println!("Day13 Pt2: {}", pt2(&input));
}

fn parse_input(input: String) -> Vec<Map2D<char>> {
    input
        .split("\n\n")
        .map(|a| Map2D::from_string(a.to_string()))
        .collect()
}

fn pt1(input: &Vec<Map2D<char>>) -> i32 {
    input.iter().fold(0, |acc, map| acc + summarize(map, 0))
}

fn pt2(input: &Vec<Map2D<char>>) -> i32 {
    input.iter().fold(0, |acc, map| acc + summarize(map, 1))
}

fn summarize(map: &Map2D<char>, errors_expected: i32) -> i32 {
    if let Some(v) = check_vertical(map, errors_expected) {
        return v;
    };

    if let Some(h) = check_horizontal(map, errors_expected) {
        return h * 100;
    };
    panic!()
}

fn check_vertical(map: &Map2D<char>, errors_expected: i32) -> Option<i32> {
    let start = map.width() / 2;
    let mut offset = 0;
    while start + offset < map.width() - 1 || start - offset >= 0 {
        'check: for left in [start - offset, start + offset] {
            if left >= 0 && left < map.width() - 1 {
                let mut sub_offset = 0;
                let mut errors_found = 0;
                loop {
                    let col_l = left + -1 * sub_offset;
                    let col_r = left + sub_offset + 1;
                    if col_l < 0 || col_r >= map.width() {
                        if errors_found != errors_expected {
                            continue 'check;
                        }
                        return Some(left + 1);
                    }

                    for y in 0..map.height() {
                        if *map.get(col_l, y).unwrap() != *map.get(col_r, y).unwrap() {
                            errors_found += 1;
                            if errors_found > errors_expected {
                                continue 'check;
                            }
                        }
                    }

                    sub_offset += 1;
                }
            }
        }
        offset += 1;
    }
    None
}

fn check_horizontal(map: &Map2D<char>, errors_expected: i32) -> Option<i32> {
    let start = map.height() / 2;
    let mut offset = 0;
    while start + offset < map.height() - 1 || start - offset >= 0 {
        'check: for top in [start - offset, start + offset] {
            if top >= 0 && top < map.height() - 1 {
                let mut sub_offset = 0;
                let mut errors_found = 0;
                loop {
                    let row_t = top + -1 * sub_offset;
                    let row_b = top + sub_offset + 1;
                    if row_t < 0 || row_b >= map.height() {
                        if errors_found != errors_expected {
                            continue 'check;
                        }
                        return Some(top + 1);
                    }
                    for x in 0..map.width() {
                        if *map.get(x, row_t).unwrap() != *map.get(x, row_b).unwrap() {
                            errors_found += 1;
                            if errors_found > errors_expected {
                                continue 'check;
                            }
                        }
                    }

                    sub_offset += 1;
                }
            }
        }
        offset += 1;
    }
    None
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
            .to_string();
        let input = parse_input(input);
        assert_eq!(pt1(&input), 405);
    }
}
