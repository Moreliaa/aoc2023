use fancy_regex::{Captures, Regex};
use itertools::Itertools;

pub fn run(input: String) {
    let input = parse_input(input);
    println!("Day4 Pt1: {}", pt1(&input));
    println!("Day4 Pt2: {}", pt2(&input));
}

fn parse_input(input: String) -> Vec<(Vec<u128>, Vec<u128>)> {
    let mut result = vec![];
    let re = Regex::new(r"Card .+: (.+) \| (.+)").unwrap();
    for l in input.lines() {
        let cap = re.captures(l).unwrap().unwrap();
        let win = parse_cap(&cap, 1);
        let have = parse_cap(&cap, 2);
        result.push((win, have));
    }
    result
}

fn parse_cap(cap: &Captures<'_>, idx: usize) -> Vec<u128> {
    cap.get(idx)
        .unwrap()
        .as_str()
        .trim()
        .split(' ')
        .into_iter()
        .filter(|a| *a != "")
        .map(|a| a.parse::<u128>().unwrap())
        .collect_vec()
}

fn pt1(input: &Vec<(Vec<u128>, Vec<u128>)>) -> u128 {
    let mut result = 0;
    for (win, have) in input {
        let mut points = 0;
        for h in have {
            for w in win {
                if w == h {
                    if points == 0 {
                        points = 1;
                    } else {
                        points *= 2;
                    }
                }
            }
        }
        result += points;
    }
    result
}

fn pt2(input: &Vec<(Vec<u128>, Vec<u128>)>) -> u128 {
    let mut result = vec![1; input.len()];
    'winnings: for (idx, (win, have)) in input.iter().enumerate() {
        let mut win_idx = idx + 1;
        let count = result[idx];
        for h in have {
            for w in win {
                if win_idx >= result.len() {
                    continue 'winnings;
                }
                if w == h {
                    result[win_idx] += 1 * count;
                    win_idx += 1;
                }
            }
        }
    }
    result.into_iter().reduce(|acc, v| acc + v).unwrap()
}
