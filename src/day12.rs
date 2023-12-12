use itertools::Itertools;
use aoc_lib::tree::Tree;

pub fn run(input: String) {
    println!("Day12 Pt1: {}", pt1(&input));
    println!("Day12 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(' ');
        let row = split.next().unwrap().chars().collect_vec();
        let numbers = split.next().unwrap().split(',').map(|n| n.parse::<i32>().unwrap()).collect_vec();
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
            for (idx, c) in p.chars().enumerate() {
                let num = numbers.get(current_num_idx);
                match num {
                    Some(val) if count > *val => { continue 'outer; },
                    None if count > 0 => { continue 'outer; },
                    _ => ()
                };
                if c == '#' {
                    last_spring = true;
                    count += 1;
                    
                } else {
                    if last_spring {
                        match num {
                            Some(val) if count != *val => { continue 'outer; },
                            None if count > 0 => { continue 'outer; },
                            _ => ()
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
                Some(val) if count != *val => { continue 'outer; },
                None if count > 0 => { continue 'outer; },
                _ => ()
            };
            sum += 1;
        }

    }
    sum
}

#[derive(Clone)]
struct Node {
    count: i32,
    current_num_idx: usize,
    last_spring: bool,
    is_failure: bool
}

#[allow(unused_assignments)]
fn pt2(input: &String) -> i32 {
    let mut sum = 0;
    for line in input.lines() {
        let mut split = line.split(' ');
        let mut row = split.next().unwrap().to_string();
        let cloned_row = row.clone();
        let cloned_row = cloned_row.as_str();
        for _ in 0..4 {
            row = row + "?" + cloned_row;
        }
        
        let row = row.chars().collect_vec();
        let mut numbers = split.next().unwrap().split(',').map(|n| n.parse::<i32>().unwrap()).collect_vec();
        let numbers_cloned = numbers.clone();
        for _ in 0..4 {
            numbers.append(&mut numbers_cloned.clone());
        }
        let mut poss = Tree::new(Node {
            count: 0,
            current_num_idx: 0,
            last_spring: false,
            is_failure: false
        });
        for (row_idx,c) in row.iter().enumerate() {
            for i in 0..poss.get_node_count() {
                if poss.get_child_ids(i).len() > 0 {
                    continue;
                }
                let mut p_count = 0;
                let mut p_current_num_idx = 0;
                let mut p_last_spring = false;
                {
                    let p = poss.get_val(i);
                    if p.is_failure {
                        continue;
                    }
                    p_count = p.count;
                    p_current_num_idx = p.current_num_idx;
                    p_last_spring = p.last_spring;
                }
                // ?
                if *c == '?' {
                    // ? -> #
                    let mut next_node = Node {
                        count: p_count + 1,
                        current_num_idx: p_current_num_idx,
                        last_spring: true,
                        is_failure: false
                    };

                    if row_idx == row.len() - 1 {
                        if p_current_num_idx < numbers.len() - 1 {
                            next_node.is_failure = true;
                        }
                        let num = numbers.get(p_current_num_idx);
                        match num {
                            Some(val) if next_node.count != *val => { next_node.is_failure = true; },
                            None if next_node.count > 0 => { next_node.is_failure = true; },
                            _ => ()
                        };
                    }

                    poss.add_child(i, next_node);

                    // ? -> .
                    let mut next_node = poss.get_val(i).clone();
                    let num = numbers.get(p_current_num_idx);
                    if p_last_spring {
                        match num {
                            Some(val) if p_count != *val => { next_node.is_failure = true; },
                            None if p_count > 0 => { next_node.is_failure = true; },
                            _ => ()
                        };
                        next_node.current_num_idx += 1;
                        next_node.last_spring = false;
                        next_node.count = 0;
                    }

                    if row_idx == row.len() - 1 {
                        if next_node.current_num_idx < numbers.len() - 1 {
                            next_node.is_failure = true;
                        }
                        let num = numbers.get(next_node.current_num_idx);
                        match num {
                            Some(val) if next_node.count != *val => { next_node.is_failure = true; },
                            None if next_node.count > 0 => { next_node.is_failure = true; },
                            _ => ()
                        };
                    }

                    poss.add_child(i, next_node);

                } else {
                    // # / .
                    let p = poss.get_mut_val(i);
                    let num = numbers.get(p_current_num_idx);
                    match num {
                        Some(val) if p_count > *val => { p.is_failure = true; continue; },
                        None if p_count > 0 => { p.is_failure = true; continue; },
                        _ => ()
                    };
                    if *c == '#' {
                        p.last_spring = true;
                        p.count += 1;
                        
                    } else {
                        if p_last_spring {
                            match num {
                                Some(val) if p_count != *val => { p.is_failure = true; continue; },
                                None if p_count > 0 => { p.is_failure = true; continue; },
                                _ => ()
                            };
                            p.current_num_idx += 1;
                            p.last_spring = false;
                            
                            p.count = 0;
                        }
                    }

                    if row_idx == row.len() - 1 {
                        if p.current_num_idx < numbers.len() - 1 {
                            p.is_failure = true;
                        }
                        let num = numbers.get(p.current_num_idx);
                        match num {
                            Some(val) if p.count != *val => { p.is_failure = true; },
                            None if p.count > 0 => { p.is_failure = true; },
                            _ => ()
                        };
                    }
                }
                
            }
        }
        for i in 0..poss.get_node_count() {
            if poss.get_child_ids(i).len() == 0 && !poss.get_val(i).is_failure {
                sum += 1;
            }
        }
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
