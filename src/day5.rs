use std::collections::HashMap;

use itertools::Itertools;

pub fn run(input: String) {
    let input = parse_input(input);
    println!("Day5 Pt1: {}", pt1(&input));
    println!("Day5 Pt2: {}", pt2(&input));
}

fn parse_input<'a>(input: String) -> (Vec<u128>, HashMap<(String, String), Vec<(u128, u128, u128)>>) {
    let mut seeds = vec![];
    let mut maps = HashMap::new();
    let mut stage = 0;
    let mut new_block = true;
    let mut current = ("".to_string(), "".to_string());
    for l in input.lines() {
        if l == "" {
            new_block = true;
            stage += 1;
            continue;
        }
        if stage == 0 {
            for (idx, s) in l.split(' ').enumerate() {
                if idx == 0 {
                    continue;
                }
                seeds.push(s.parse::<u128>().unwrap());
            }
        } else {
            if new_block {
                let mut l_split = l.split(' ').next().unwrap().split('-');
                let source = l_split.next().unwrap().to_string();
                l_split.next();
                let dest = l_split.next().unwrap().to_string();
                maps.insert((source.clone(), dest.clone()), vec![]);
                current = (source, dest);
                new_block = false;
            } else {
                println!("{l}");
                let l_split = l.split(' ').map(|v| v.parse::<u128>().unwrap()).collect_tuple().unwrap();
                maps.entry(current.clone()).and_modify(|c| c.push(l_split));
            }
        }
    }
    (seeds, maps)
}

fn pt1(input: &(Vec<u128>, HashMap<(String, String), Vec<(u128, u128, u128)>>)) -> u128 {
    let mut lowest_loc: Option<u128> = None;
    for seed in input.0.iter() {
        let loc = convert(&input, &"seed".to_string(), &"location".to_string(), *seed);
        lowest_loc = match lowest_loc {
            Some(val) => Some(val.min(loc)),
            None => Some(loc)
        }; 
    }
    lowest_loc.unwrap()
}

fn convert(input: &(Vec<u128>, HashMap<(String, String), Vec<(u128, u128, u128)>>), source: &String, target: &String, number: u128) -> u128 {
    for (s, d) in input.1.keys() {
        if source == s {
            //println!("S: {s} D: {d} Source: {source} Target: {target} Number: {number}");
            for (drs, srs, len) in input.1.get(&(s.clone(),d.clone())).unwrap() {
                if *srs <= number && number < srs + len {
                    let offset = number - srs;
                    let next_number = drs + offset;
                    if target == d {
                        return next_number;
                    } else {
                        return convert(input, d, target, next_number);
                    }
                }
            }
            if target == d {
                return number;
            } else {
                return convert(input, d, target, number);
            }
        }
    }
    panic!();
}

fn pt2(input: &(Vec<u128>, HashMap<(String, String), Vec<(u128, u128, u128)>>)) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4".to_string();
        let input = parse_input(input);
        assert_eq!(pt1(&input), 35);
    }
}
