use fancy_regex::Regex;
use std::collections::HashMap;

const REJECT: &str = "R";
const ACCEPT: &str = "A";

pub fn run(input: String) {
    let (workflows, parts) = parse_input(&input);
    println!("Day19 Pt1: {}", pt1(&workflows, &parts));
    println!("Day19 Pt2: {}", pt2(&workflows));
    // 11370283072461387 too high
}

struct Rule<'a> {
    attr: &'a str,
    op: &'a str,
    val: i32,
    result: &'a str,
}

struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

fn parse_input<'a>(input: &'a String) -> (HashMap<&'a str, Vec<Rule>>, Vec<Part>) {
    let re_workflow = Regex::new(r"(.+){(.+)}").unwrap();
    let re_rule = Regex::new(r"(.)(.)(\d+):(.+)").unwrap(); // attribute, operator, number, workflow/accept/reject
    let re_ratings = Regex::new(r"{x=(.+),m=(.+),a=(.+),s=(.+)}").unwrap();

    let mut workflows: HashMap<&str, Vec<Rule>> = HashMap::new();
    let mut parts: Vec<Part> = vec![];
    let mut stage = 0;
    for l in input.lines() {
        if l == "" {
            stage += 1;
            continue;
        }
        if stage == 0 {
            // workflows
            let cap = re_workflow.captures(l).unwrap().unwrap();
            let name = cap.get(1).unwrap().as_str();
            let rules = cap.get(2).unwrap().as_str();
            workflows.insert(name, vec![]);
            let entry = workflows.get_mut(name).unwrap();
            for r in rules.split(',') {
                let mut cap_r = re_rule.captures(r).unwrap();
                if let Some(capped) = cap_r.take() {
                    entry.push(Rule {
                        attr: capped.get(1).unwrap().as_str(),
                        op: capped.get(2).unwrap().as_str(),
                        val: capped.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                        result: capped.get(4).unwrap().as_str(),
                    })
                } else {
                    // accept / reject
                    entry.push(Rule {
                        attr: "x",
                        op: ">",
                        val: i32::MIN,
                        result: r,
                    });
                };
            }
        } else {
            // ratings
            let cap = re_ratings.captures(l).unwrap().unwrap();
            parts.push(Part {
                x: cap.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                m: cap.get(2).unwrap().as_str().parse::<i32>().unwrap(),
                a: cap.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                s: cap.get(4).unwrap().as_str().parse::<i32>().unwrap(),
            });
        }
    }
    (workflows, parts)
}

fn pt1(workflows: &HashMap<&str, Vec<Rule>>, parts: &Vec<Part>) -> i32 {
    let mut sum = 0;
    for p in parts {
        if check_part(workflows, p) {
            sum += p.x + p.m + p.a + p.s;
        }
    }
    sum
}

fn check_part(workflows: &HashMap<&str, Vec<Rule>>, part: &Part) -> bool {
    let mut current_workflow = "in";
    loop {
        for r in workflows.get(current_workflow).unwrap() {
            let val_part = match r.attr {
                "x" => part.x,
                "m" => part.m,
                "a" => part.a,
                "s" => part.s,
                _ => panic!(),
            };
            let success = match r.op {
                ">" => val_part > r.val,
                "<" => val_part < r.val,
                _ => panic!(),
            };

            if !success {
                continue;
            }

            match r.result {
                ACCEPT => return true,
                REJECT => return false,
                next_workflow => {
                    current_workflow = next_workflow;
                    break;
                }
            }
        }
    }
}

#[derive(Clone)]
struct State {
    x_min: i32,
    m_min: i32,
    a_min: i32,
    s_min: i32,
    x_max: i32,
    m_max: i32,
    a_max: i32,
    s_max: i32,
}

fn pt2(workflows: &HashMap<&str, Vec<Rule>>) -> i128 {
    work(
        workflows,
        "in",
        State {
            x_min: 1,
            m_min: 1,
            a_min: 1,
            s_min: 1,
            x_max: 4000,
            m_max: 4000,
            a_max: 4000,
            s_max: 4000,
        },
    )
}

fn work(workflows: &HashMap<&str, Vec<Rule>>, current_workflow: &str, state: State) -> i128 {
    match current_workflow {
        ACCEPT => {
            return ((state.x_max - state.x_min + 1) as i128
                * (state.m_max - state.m_min + 1) as i128
                * (state.a_max - state.a_min + 1) as i128
                * (state.s_max - state.s_min + 1) as i128) as i128
        }
        REJECT => return 0,
        _ => (),
    };

    let mut acc = 0;
    for r in workflows.get(current_workflow).unwrap() {
        match r.result {
            ACCEPT => {
                acc += ((state.x_max - state.x_min + 1) as i128
                    * (state.m_max - state.m_min + 1) as i128
                    * (state.a_max - state.a_min + 1) as i128
                    * (state.s_max - state.s_min + 1) as i128) as i128;
                continue;
            }
            REJECT => continue,
            _ => (),
        };

        match r.op {
            ">" => {
                // check success path
                let mut sub_state = state.clone();
                match r.attr {
                    "x" if state.x_max > r.val => {
                        sub_state.x_min = r.val.max(sub_state.x_min);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "m" if state.m_max > r.val => {
                        sub_state.m_min = r.val.max(sub_state.m_min);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "a" if state.a_max > r.val => {
                        sub_state.a_min = r.val.max(sub_state.a_min);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "s" if state.s_max > r.val => {
                        sub_state.s_min = r.val.max(sub_state.s_min);
                        acc += work(workflows, r.result, sub_state);
                    }
                    _ => panic!(),
                };
            }
            "<" => {
                // check success path
                let mut sub_state = state.clone();
                match r.attr {
                    "x" if state.x_min < r.val => {
                        sub_state.x_max = r.val.min(sub_state.x_max);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "m" if state.m_min < r.val => {
                        sub_state.m_max = r.val.min(sub_state.m_max);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "a" if state.a_min < r.val => {
                        sub_state.a_max = r.val.min(sub_state.a_max);
                        acc += work(workflows, r.result, sub_state);
                    }
                    "s" if state.s_min < r.val => {
                        sub_state.s_max = r.val.min(sub_state.s_max);
                        acc += work(workflows, r.result, sub_state);
                    }
                    _ => panic!(),
                };
            }
            _ => panic!(),
        };
    }
    acc
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        run("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"
            .to_string());
    }
}
