use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn run(input: String) {
    println!("Day25 Pt1: {}", pt1(&input));
    println!("Day25 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
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
    //let mut comp: HashMap<&str, HashMap<&str, i32>> = HashMap::new();
    /*comp.insert("1" , HashMap::from([("2", 2), ("5", 3)]));
    comp.insert("2" , HashMap::from([("1", 2), ("3", 3), ("5", 2), ("6", 2)]));
    comp.insert("3" , HashMap::from([("2", 3),("4", 4),("7", 2)]));
    comp.insert("4" , HashMap::from([("3", 4),("7", 2),("8", 2),]));
    comp.insert("5" , HashMap::from([("1", 3),("2", 2),("6", 3),]));
    comp.insert("6" , HashMap::from([("2", 2),("5", 3),("7", 1),]));
    comp.insert("7" , HashMap::from([("3", 2),("4", 2),("6", 1),("8", 3),]));
    comp.insert("8" , HashMap::from([("4", 2),("7", 3),]));*/
    //start_node = "2";

    let node_count_total = comp.len();

    // Stoer-Wagner

    let mut current_best_weight = std::i32::MAX;
    let mut current_best_len = node_count_total;
    while comp.len() > 2 {
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
        // merge t into s
        let t = weights.last().unwrap();
        let s = weights[weights.len() - 2];
        let weight_cut = t.1;

        if weight_cut < current_best_weight {
            current_best_len = comp.len();
            current_best_weight = weight_cut;
            //print!("BEST CUT ");
        }
        //println!("{:?} {} {weight_cut}", weights, comp.len());

        //println!{"t: {} {:?}", t.0, comp_binding.get(t.0).unwrap()};
        for (other, w) in comp_binding.get(t.0).unwrap() {
            //println!("s: {} {:?} other: {} {:?}", s.0, comp.get(s.0).unwrap(), other, comp.get(other).unwrap());
            
            comp.entry(s.0).and_modify(|e| {
                e.entry(other).and_modify(|e2| { *e2 += w; } ).or_insert(*w);
                e.remove(t.0);
            });
            comp.entry(other).and_modify(|e| {
                e.entry(s.0).and_modify(|e2| { *e2 += w; } ).or_insert(*w);
                e.remove(t.0);
            });
        }
        comp.entry(s.0).and_modify(|e| {e.remove(s.0);});
        comp.remove(t.0);
        //println!("{:?}", comp);
    }

    (current_best_len * (node_count_total - current_best_len)) as i32 // > 480654
    // > 542784
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

        let input = "1: 2 5
2: 3 5 6
3: 4 7
4: 7 8
5: 6
6: 7
7: 8".to_string();
        println!("{}", pt1(&input));
    }
}
