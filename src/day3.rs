use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    let map = Map2D::from_string(input);
    println!("Day3 Pt1: {}", pt1(&map));
    println!("Day3 Pt2: {}", pt2(&map));
}

fn pt1(input: &Map2D<char>) -> u32 {
    let mut sum = 0;
    let mut num: Vec<(i32, i32)> = vec![];
    for y in 0..input.height() {
        for x in 0..input.width() {
            let c = input.get(x, y).unwrap();
            match c {
                '0'..='9' => num.push((x, y)),
                _ => {
                    if num.len() > 0 && is_part(input, &num) {
                        sum += sum_digits(input, &num)
                    }
                    num = vec![];
                }
            }
        }
    }
    sum
}

fn sum_digits(input: &Map2D<char>, num: &Vec<(i32, i32)>) -> u32 {
    let mut sum = 0;
    for (i, v) in num.iter().enumerate() {
        sum += input.get(v.0, v.1).unwrap().to_digit(10).unwrap()
            * (10 as u32).pow((num.len() - i - 1) as u32);
    }
    sum
}

fn is_part(input: &Map2D<char>, num: &Vec<(i32, i32)>) -> bool {
    for n in num {
        let symbols =
            input.aggregate_range(n.0 - 1, n.0 + 1, n.1 - 1, n.1 + 1, |v, _, _| match *v {
                '0'..='9' | '.' => 0,
                _ => 1,
            });
        if symbols > 0 {
            return true;
        }
    }
    false
}

fn pt2(input: &Map2D<char>) -> u32 {
    let mut sum = 0;
    for y in 0..input.height() {
        for x in 0..input.width() {
            let c = input.get(x, y).unwrap();
            match c {
                '*' => {
                    let adjacent = get_adjacent_nums(input, x, y);
                    if adjacent.len() == 2 {
                        sum += adjacent[0] * adjacent[1];
                    }
                }
                _ => (),
            }
        }
    }
    sum
}

fn get_adjacent_nums(input: &Map2D<char>, x_gear: i32, y_gear: i32) -> Vec<u32> {
    let mut result = vec![];
    let mut num: Vec<(i32, i32)> = vec![];
    for y in y_gear - 1..=y_gear + 1 {
        for x in 0..input.width() {
            match input.get(x, y) {
                Some(val) => match *val {
                    '0'..='9' => num.push((x, y)),
                    _ => {
                        if num.len() > 0 && is_adjacent(input, &num, x_gear, y_gear) {
                            result.push(sum_digits(input, &num));
                        }
                        num = vec![];
                    }
                },
                None => {
                    if num.len() > 0 && is_adjacent(input, &num, x_gear, y_gear) {
                        result.push(sum_digits(input, &num));
                    }
                    num = vec![];
                }
            };
        }
    }
    if num.len() > 0 && is_adjacent(input, &num, x_gear, y_gear) {
        result.push(sum_digits(input, &num));
    }
    num = vec![];
    result
}

fn is_adjacent(input: &Map2D<char>, num: &Vec<(i32, i32)>, x_gear: i32, y_gear: i32) -> bool {
    for n in num {
        let symbols = input.aggregate_range(n.0 - 1, n.0 + 1, n.1 - 1, n.1 + 1, |_, x, y| {
            if x == x_gear && y == y_gear {
                1
            } else {
                0
            }
        });
        if symbols > 0 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = String::from(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..",
        );
        let map = Map2D::from_string(input);
        println!("{}", pt2(&map));
    }
}
