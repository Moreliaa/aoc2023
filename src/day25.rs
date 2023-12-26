use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn run(input: String) {
    println!("Day25 Pt1: {}", pt1(&input));
    println!("Day25 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let mut comp: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut comp_no_dupes: HashMap<&str, HashSet<&str>> = HashMap::new();
    for l in input.lines() {
        let mut split = l.split(": ");
        let c = split.next().unwrap();
        for other in split.next().unwrap().split(" ") {
            comp.entry(c).and_modify(|e| {e.insert(other);}).or_insert(HashSet::from([other]));
            comp.entry(other).and_modify(|e| {e.insert(c);}).or_insert(HashSet::from([c]));
            
            comp_no_dupes.entry(c).and_modify(|e| {e.insert(other);}).or_insert(HashSet::from([other]));
        }
    }
    let mut counter = 0;
    for (key1, val1) in comp_no_dupes.iter() {
        for (key2, val2) in comp_no_dupes.iter() {
            for (key3, val3) in comp_no_dupes.iter() {
                for conn1 in val1 {
                    for conn2 in val2 {
                        for conn3 in val3 {
                            counter += 1;
                            if counter % 100000 == 0 {
                                println!("{counter}");
                            }
                            let mut comp_cloned = comp.clone();
                            comp_cloned.entry(key1).and_modify(|e| {e.remove(conn1);});
                            comp_cloned.entry(conn1).and_modify(|e| {e.remove(key1);});
                            
                            let mut skip = false;
                            comp_cloned.entry(key2).and_modify(|e| {
                                let was_removed = e.remove(conn2);
                                if !was_removed {
                                    skip = true;
                                }
                            });
                            comp_cloned.entry(conn2).and_modify(|e| {
                                let was_removed = e.remove(key2);
                                if !was_removed {
                                    skip = true;
                                }
                            });

                            comp_cloned.entry(key3).and_modify(|e| {
                                let was_removed = e.remove(conn3);
                                if !was_removed {
                                    skip = true;
                                }
                            });
                            comp_cloned.entry(conn3).and_modify(|e| {
                                let was_removed = e.remove(key3);
                                if !was_removed {
                                    skip = true;
                                }
                            });
                            if skip {
                                continue;
                            }
                            let g = find_groups(&comp_cloned);
                            if g.len() == 2 {
                                println!("{key1}->{conn1} {key2}->{conn2} {key3}->{conn3} {:?}", g);
                                return g.into_iter().fold(1, |acc, val| acc * val);
                            }
                        }
                    }
                }
            }
        }   
    }
    /*for i in 0..wires.len() - 2 {
        for j in i + 1..wires.len() - 1 {
            for k in j + 1..wires.len() {
                let mut comp_cloned = comp.clone();
                comp_cloned.remove(wires[i]);
                comp_cloned.remove(wires[j]);
                comp_cloned.remove(wires[k]);
                for (_, v) in comp_cloned.iter_mut() {
                    v.remove(wires[i]);
                    v.remove(wires[j]);
                    v.remove(wires[k]);
                }
                let g = find_groups(&cloned_comp);
                if g.len() == 2 {
                    return g.into_iter().fold(1, |acc, val| acc * val);
                }
            }
        }
    }*/
    panic!();

    
}

fn find_groups(comp: &HashMap<&str, HashSet<&str>>) -> Vec<i32> {
    let mut comp_remaining = comp.keys().collect::<HashSet<&&str>>();
    let mut groups = vec![];

    while comp_remaining.len() > 0 {
        let first_comp = (*comp_remaining.iter().next().unwrap()).clone();
        comp_remaining.remove(&first_comp);
        let mut next_comp = vec![first_comp];
        let mut size = 0;
        while let Some(wire) = next_comp.pop() {
            size += 1;
            for other in comp.get(wire).unwrap() {
                if comp_remaining.contains(&other) {
                    next_comp.push(comp_remaining.take(other).unwrap());
                }
            }
        }
        groups.push(size);
    }

    groups
}

fn pt2(input: &String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr".to_string();
        assert_eq!(pt1(&input), 54);
    }
}
