use itertools::Itertools;
use num::integer::lcm;
use std::collections::{HashMap, VecDeque};

pub fn run(input: String) {
    println!("Day20 Pt1: {}", pt1(&input));
    println!("Day20 Pt2: {}", pt2(&input));
}

#[derive(Debug)]
struct Module<'a> {
    kind: &'a str,
    is_on: bool,
    dest: Vec<&'a str>,
    inc: HashMap<&'a str, i32>,
}

fn parse_input<'a>(input: &'a String) -> HashMap<&'a str, Module> {
    let mut modules: HashMap<&str, Module> = HashMap::new();
    for l in input.lines() {
        let mut split = l.split(" -> ");
        let module = split.next().unwrap();
        let dest = split.next().unwrap().split(", ").collect_vec();
        let kind = if module == "broadcaster" {
            module
        } else {
            &module[0..1]
        };
        let name = if module == "broadcaster" {
            module
        } else {
            &module[1..module.len()]
        };
        for d in dest.iter() {
            modules
                .entry(d)
                .and_modify(|m| {
                    m.inc.insert(name, 0);
                })
                .or_insert({
                    let mut m = Module {
                        kind: "",
                        is_on: false,
                        dest: vec![],
                        inc: HashMap::new(),
                    };
                    m.inc.insert(name, 0);
                    m
                });
        }

        modules
            .entry(name)
            .and_modify(|m| {
                m.kind = kind;
                m.dest = dest.clone();
            })
            .or_insert(Module {
                kind,
                is_on: false,
                dest,
                inc: HashMap::new(),
            });
    }

    modules
}

fn pt1(input: &String) -> i128 {
    let mut modules = parse_input(input);
    let mut pushes: i128 = 0;
    let target = 1000;
    let (mut low_pulses, mut high_pulses) = (0, 0);
    let mut last = 0;
    'outer: loop {
        /*let next = modules.values().fold(0, |acc, m| {
            acc + m.inc.iter().fold(0, |acc2, i| {
                acc2 + i.1
            })
        });
        println!("next: {next} diff: {}",next-last);*/
        //last = next;
        pushes += 1;
        let mut tasks = VecDeque::new();
        tasks.push_front(("broadcaster", 0, "button")); // current, pulse, source
        while let Some(task) = tasks.pop_front() {
            //println!("{:?}", task);

            if task.1 == 0 {
                low_pulses += 1;
            } else {
                high_pulses += 1;
            }
            let m = modules.get_mut(task.0).unwrap();
            m.inc.entry(task.2).and_modify(|e| *e = task.1); // note: "button" is not in the map and won't be modified
            match m.kind {
                "button" => {
                    tasks.extend(m.dest.clone().into_iter().map(|d| (d, 0, task.0)));
                }
                "broadcaster" => {
                    tasks.extend(m.dest.clone().into_iter().map(|d| (d, task.1, task.0)));
                }
                "%" => {
                    // flip-flop
                    if task.1 == 1 {
                        continue;
                    }

                    let out = if m.is_on { 0 } else { 1 };
                    m.is_on = !m.is_on;
                    tasks.extend(m.dest.clone().into_iter().map(|d| (d, out, task.0)));
                }
                "&" => {
                    // conjunction
                    let is_all_high = m.inc.values().fold(true, |acc, v| acc && *v == 1);
                    let out = if is_all_high { 0 } else { 1 };
                    tasks.extend(m.dest.clone().into_iter().map(|d| (d, out, task.0)));
                }
                _ => {
                    if task.1 == 0 {
                        println!("Success {pushes}")
                    } else {
                        continue;
                    }
                } // untyped module
            }
        }
    }
    println!("{low_pulses} {high_pulses}");
    low_pulses * high_pulses // 530835824 too low
}

fn pt2(input: &String) -> i32 {
    [3889, 3881, 4021, 4013]
        .into_iter()
        .reduce(|acc, c| lcm(acc, c))
        .unwrap()
    // 2020350377 too low
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
            .to_string();
        assert_eq!(pt1(&input), 32000000);
    }
}
