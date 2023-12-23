use itertools::Itertools;
use std::collections::HashMap;

pub fn run(input: String) {
    println!("Day12 Pt1: {}", pt1(&input));
    println!("Day12 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(' ');
        let row = split.next().unwrap().chars().collect_vec();
        let numbers = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<i32>().unwrap())
            .collect_vec();
        let mut poss = vec![];
        for c in row {
            if c == '?' {
                if poss.len() == 0 {
                    poss.push(".".to_string());
                    poss.push("#".to_string());
                } else {
                    let mut next = vec![];
                    for p in poss {
                        next.push(p.clone() + ".");
                        next.push(p + "#");
                    }
                    poss = next;
                }
            } else {
                if poss.len() == 0 {
                    poss.push(c.to_string());
                } else {
                    for p in poss.iter_mut() {
                        p.push(c);
                    }
                }
            }
        }

        'outer: for p in poss {
            let mut count = 0;
            let mut current_num_idx = 0;
            let mut last_spring = false;
            for (_, c) in p.chars().enumerate() {
                let num = numbers.get(current_num_idx);
                match num {
                    Some(val) if count > *val => {
                        continue 'outer;
                    }
                    None if count > 0 => {
                        continue 'outer;
                    }
                    _ => (),
                };
                if c == '#' {
                    last_spring = true;
                    count += 1;
                } else {
                    if last_spring {
                        match num {
                            Some(val) if count != *val => {
                                continue 'outer;
                            }
                            None if count > 0 => {
                                continue 'outer;
                            }
                            _ => (),
                        };
                        current_num_idx += 1;
                        last_spring = false;

                        count = 0;
                    }
                }
            }
            if current_num_idx < numbers.len() - 1 {
                continue;
            }
            let num = numbers.get(current_num_idx);
            match num {
                Some(val) if count != *val => {
                    continue 'outer;
                }
                None if count > 0 => {
                    continue 'outer;
                }
                _ => (),
            };
            sum += 1;
        }
    }
    sum
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Node {
    count: u8,
    current_num_idx: u8,
    last_spring: bool,
    is_failure: bool,
}

#[allow(unused_assignments)]
fn pt2(input: &String) -> i128 {
    let mut sum = 0;
    for (line_idx, line) in input.lines().enumerate() {
        println!("Line: {line_idx}");
        let mut split = line.split(' ');
        let mut row = split.next().unwrap().to_string();
        let cloned_row = row.clone();
        let cloned_row = cloned_row.as_str();
        for _ in 0..4 {
            row = row + "?" + cloned_row;
        }

        let row = row.chars().collect_vec();
        let mut numbers = split
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse::<u8>().unwrap())
            .collect_vec();
        let numbers_cloned = numbers.clone();
        for _ in 0..4 {
            numbers.append(&mut numbers_cloned.clone());
        }
        let mut poss: HashMap<Node, i128> = HashMap::new();
        poss.insert(
            Node {
                count: 0,
                current_num_idx: 0,
                last_spring: false,
                is_failure: false,
            },
            1,
        );
        let mut next_poss: HashMap<Node, i128> = HashMap::new();
        let row_len = row.len();
        for (row_idx, c) in row.iter().enumerate() {
            println! {"row_len: {row_len} row_idx: {row_idx} ids: {:?}", poss.len()};
            for (mut p, instances) in poss {
                let mut p_count = 0;
                let mut p_current_num_idx = 0;
                let mut p_last_spring = false;
                {
                    if p.is_failure {
                        panic!();
                    }
                    p_count = p.count;
                    p_current_num_idx = p.current_num_idx;
                    p_last_spring = p.last_spring;
                }

                let mut remaining_numbers_sum = 0;
                for i_r in p_current_num_idx + 1..numbers.len() as u8 {
                    remaining_numbers_sum += numbers[i_r as usize];
                }
                if numbers.len() as u8 > p_current_num_idx {
                    remaining_numbers_sum += numbers.len() as u8 - p_current_num_idx - 1;
                }
                if remaining_numbers_sum > (row_len - row_idx) as u8 {
                    continue;
                }

                // ?
                if *c == '?' {
                    // ? -> #
                    let mut next_node = Node {
                        count: p_count + 1,
                        current_num_idx: p_current_num_idx,
                        last_spring: true,
                        is_failure: false,
                    };

                    if row_idx == row.len() - 1 {
                        if p_current_num_idx < numbers.len() as u8 - 1 {
                            next_node.is_failure = true;
                        }
                        let num = numbers.get(p_current_num_idx as usize);
                        match num {
                            Some(val) if next_node.count != *val => {
                                next_node.is_failure = true;
                            }
                            None if next_node.count > 0 => {
                                next_node.is_failure = true;
                            }
                            _ => (),
                        };
                    }

                    if !next_node.is_failure {
                        next_poss
                            .entry(next_node)
                            .and_modify(|v| {
                                *v += instances;
                            })
                            .or_insert(instances);
                    }

                    // ? -> .
                    let mut next_node = p.clone();
                    let num = numbers.get(p_current_num_idx as usize);
                    if p_last_spring {
                        match num {
                            Some(val) if p_count != *val => {
                                next_node.is_failure = true;
                            }
                            None if p_count > 0 => {
                                next_node.is_failure = true;
                            }
                            _ => (),
                        };
                        next_node.current_num_idx += 1;
                        next_node.last_spring = false;
                        next_node.count = 0;
                    }

                    if row_idx == row.len() - 1 {
                        if next_node.current_num_idx < numbers.len() as u8 - 1 {
                            next_node.is_failure = true;
                        }
                        let num = numbers.get(next_node.current_num_idx as usize);
                        match num {
                            Some(val) if next_node.count != *val => {
                                next_node.is_failure = true;
                            }
                            None if next_node.count > 0 => {
                                next_node.is_failure = true;
                            }
                            _ => (),
                        };
                    }
                    if !next_node.is_failure {
                        next_poss
                            .entry(next_node)
                            .and_modify(|v| {
                                *v += instances;
                            })
                            .or_insert(instances);
                    }
                } else {
                    // # / .
                    let num = numbers.get(p_current_num_idx as usize);
                    match num {
                        Some(val) if p_count > *val => {
                            p.is_failure = true;
                        }
                        None if p_count > 0 => {
                            p.is_failure = true;
                        }
                        _ => (),
                    };
                    if *c == '#' {
                        p.last_spring = true;
                        p.count += 1;
                    } else {
                        if p_last_spring {
                            match num {
                                Some(val) if p_count != *val => {
                                    p.is_failure = true;
                                }
                                None if p_count > 0 => {
                                    p.is_failure = true;
                                }
                                _ => (),
                            };
                            p.current_num_idx += 1;
                            p.last_spring = false;

                            p.count = 0;
                        }
                    }

                    if row_idx == row.len() - 1 {
                        if p.current_num_idx < numbers.len() as u8 - 1 {
                            p.is_failure = true;
                        }
                        let num = numbers.get(p.current_num_idx as usize);
                        match num {
                            Some(val) if p.count != *val => {
                                p.is_failure = true;
                            }
                            None if p.count > 0 => {
                                p.is_failure = true;
                            }
                            _ => (),
                        };
                    }
                    if !p.is_failure {
                        next_poss
                            .entry(p.clone())
                            .and_modify(|v| {
                                *v += instances;
                            })
                            .or_insert(instances);
                    }
                }
            }
            poss = next_poss;
            next_poss = HashMap::new();
        }
        println! {"row_len: {row_len} ids: {:?}", poss.len()};
        sum += poss.values().fold(0, |acc, v| acc + v);
    }
    sum
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "???.### 1,1,3".to_string();
        assert_eq!(pt1(&input), 1);
        let input = ".??..??...?##. 1,1,3".to_string();
        assert_eq!(pt1(&input), 4);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6".to_string();
        assert_eq!(pt1(&input), 1);
        let input = "????.#...#... 4,1,1".to_string();
        assert_eq!(pt1(&input), 1);
        let input = "????.######..#####. 1,6,5".to_string();
        assert_eq!(pt1(&input), 4);
        let input = "?###???????? 3,2,1".to_string();
        assert_eq!(pt1(&input), 10);

        let input = "???.### 1,1,3".to_string();
        assert_eq!(pt2(&input), 1);
        let input = ".??..??...?##. 1,1,3".to_string();
        assert_eq!(pt2(&input), 16384);
        let input = "?#?#?#?#?#?#?#? 1,3,1,6".to_string();
        assert_eq!(pt2(&input), 1);
        let input = "????.#...#... 4,1,1".to_string();
        assert_eq!(pt2(&input), 16);
        let input = "????.######..#####. 1,6,5".to_string();
        assert_eq!(pt2(&input), 2500);
        let input = "?###???????? 3,2,1".to_string();
        assert_eq!(pt2(&input), 506250);
    }
}
