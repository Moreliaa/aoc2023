use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn run(input: String) {
    println!("Day25 Pt1: {}", pt1(&input, 2));
    println!("Day25 Pt2: {}", pt2(&input));
}

fn pt1(input: &String, target_vertices: usize) -> i32 {
    let mut comp: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    let mut start_node = "";
    for l in input.lines() {
        let mut split = l.split(": ");
        let c = split.next().unwrap();
        start_node = c;
        for other in split.next().unwrap().split(" ") {
            comp.entry(c).and_modify(|e| {e.insert(other, 1);}).or_insert(HashMap::from([(other, 1)]));
            comp.entry(other).and_modify(|e| {e.insert(c, 1);}).or_insert(HashMap::from([(c, 1)]));
        }
    }

    let node_count_total = comp.len();

    // Stoer-Wagner

    let mut current_best_weight = std::i32::MAX;
    let mut current_best_len = node_count_total;
    while comp.len() >= target_vertices {
        println!("{}", comp.len());
        // Maximum Adjacency Search
        let comp_binding = comp.clone();
        let mut candidates: HashSet<&&str> = comp_binding.keys().collect();
        candidates.remove(&start_node);
        let mut seen_candidates: HashSet<&&str> = HashSet::from([&start_node]);

        let mut weights: Vec<(&&str, i32)> = vec![];
        while candidates.len() > 0 {
            let mut max_candidate = None;
            let mut max_weight = std::i32::MIN;
            for c in candidates.iter() {
                let mut weight = 0;
                for (other, w) in comp.get(*c).unwrap() {
                    if seen_candidates.contains(other) {
                        weight += w;
                    }
                }
                if weight > max_weight {
                    max_weight = weight;
                    max_candidate = Some(c.clone());
                }
            }
            let max_candidate = max_candidate.unwrap();
            weights.push((max_candidate, max_weight));
            candidates.remove(max_candidate);
            seen_candidates.insert(&max_candidate);
        }
        // merge s into t
        let t = weights.last().unwrap();
        let s = weights.last().unwrap();
        let weight_cut = t.1;

        for (other, w) in comp_binding.get(s.0).unwrap() {
            comp.entry(t.0).and_modify(|e| {
                e.entry(other).and_modify(|e2| { *e2 += w; } ).or_insert(1);
            });
            comp.entry(other).and_modify(|e| {
                e.entry(t.0).and_modify(|e2| { *e2 += w; } ).or_insert(1);
                e.remove(s.0);
            });
        }
        comp.remove(s.0);

        if weight_cut < current_best_weight {
            current_best_len = comp.len();
            current_best_weight = weight_cut;
            println!("{:?}", seen_candidates);
        }
    }
    (current_best_len * (node_count_total - current_best_len)) as i32 // > 480654
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
        assert_eq!(pt1(&input, 2), 54);
    }
}
