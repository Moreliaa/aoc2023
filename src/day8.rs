use std::collections::HashMap;
use num::integer::lcm;
use fancy_regex::Regex;
use itertools::Itertools;

pub fn run(input: String) {
    let (instr, map) = parse_input(&input);
    println!("Day8 Pt1: {}", pt1(&instr, &map));
    println!("Day8 Pt2: {}", pt2(&instr, &map));
}

fn parse_input<'a>(input: &'a String) -> (Vec<char>, HashMap<&'a str, (&'a str, &'a str)>) {
    let mut lines = input.lines();
    let instr: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let re = Regex::new(r"(.+) = \((.+), (.+)\)").unwrap();
    while let Some(line) = lines.next() {
        let cap = re.captures(line).unwrap().unwrap();
        let n = cap.get(1).unwrap().as_str();
        let l = cap.get(2).unwrap().as_str();
        let r = cap.get(3).unwrap().as_str();
        map.insert(n, (l, r));
    }
    (instr, map)
}

fn pt1(instr: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> u128 {
    steps(instr, map, "AAA")
}

fn pt2(instr: &Vec<char>, map: &HashMap<&str, (&str, &str)>) -> u128 {
    let loc = map.keys().filter(|k| &k[2..] == "A").collect_vec();
    let steps = loc.iter().map(|l| steps(instr, map, l)).collect_vec();
    steps.into_iter().reduce(|acc, c| lcm(acc, c)).unwrap()
}

fn steps(instr: &Vec<char>, map: &HashMap<&str, (&str, &str)>, start_loc: &str) -> u128 {
    let mut loc = start_loc;
    let mut idx = 0;
    let mut steps = 0;
    while &loc[2..] != "Z" {
        if idx == instr.len() {
            idx = 0;
        }
        let n = map.get(loc).unwrap();
        loc = if instr[idx] == 'L' { n.0 } else { n.1 };
        idx += 1;
        steps += 1;
    }
    steps
}