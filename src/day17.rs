use aoc_lib::map2d::Map2D;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let input = Map2D::from_string(input);
    println!("Day17 Pt1: {}", pt1(&input, true));
    println!("Day17 Pt2: {}", pt1(&input, false));
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    x: i32,
    y: i32,
    dir: char,
    straight: i32,
}

fn pt1(input: &Map2D<char>, is_pt1: bool) -> i32 {
    let initial_nodes = [
        Node {
            x: 0,
            y: 0,
            dir: 'E',
            straight: 0,
        },
        Node {
            x: 0,
            y: 0,
            dir: 'S',
            straight: 0,
        },
    ];

    let mut unseen: HashSet<Node> = HashSet::new();
    let mut nodes: HashMap<Node, i32> = HashMap::new();
    initial_nodes.iter().for_each(|n| {
        nodes.insert(n.clone(), 0);
    });
    initial_nodes.into_iter().for_each(|n| {
        unseen.insert(n);
    });

    loop {
        // find next node
        let current = {
            let mut min = std::i32::MAX;
            let mut sub_result = None;
            for n in unseen.iter() {
                let other = *nodes.get(&n).unwrap();
                if other < min {
                    min = other;
                    sub_result = Some(n.clone());
                }
            }
            println!("{} {}", unseen.len(), nodes.len());
            sub_result.unwrap()
        };
        let current = unseen.take(&current).unwrap();

        if current.x == input.width() - 1 && current.y == input.height() - 1 {
            if is_pt1 {
                return *nodes.get(&current).unwrap();
            } else {
                if current.straight >= 4 {
                    return *nodes.get(&current).unwrap();
                }
            }
        }

        // travel
        let left_dir = match current.dir {
            'N' => 'W',
            'E' => 'N',
            'S' => 'E',
            'W' => 'S',
            _ => panic!(),
        };

        let right_dir = match current.dir {
            'N' => 'E',
            'E' => 'S',
            'S' => 'W',
            'W' => 'N',
            _ => panic!(),
        };

        travel(current, left_dir, input, &mut nodes, &mut unseen, is_pt1);
        travel(current, right_dir, input, &mut nodes, &mut unseen, is_pt1);
        travel(current, current.dir, input, &mut nodes, &mut unseen, is_pt1);
    }
}

fn travel(
    current: Node,
    dir: char,
    input: &Map2D<char>,
    nodes: &mut HashMap<Node, i32>,
    unseen: &mut HashSet<Node>,
    is_pt1: bool,
) {
    let min_straight = if is_pt1 { 0 } else { 4 };
    if dir != current.dir && current.straight < min_straight {
        return;
    }

    let max_straight = if is_pt1 { 3 } else { 10 };
    let straight = if dir == current.dir {
        current.straight + 1
    } else {
        1
    };
    if straight > max_straight {
        return;
    }

    let coords = match dir {
        'N' => (0, -1),
        'E' => (1, 0),
        'S' => (0, 1),
        'W' => (-1, 0),
        _ => panic!(),
    };

    let x = current.x + coords.0;
    let y = current.y + coords.1;

    if let Some(tile) = input.get(x, y) {
        let mut heat = nodes.get(&current).unwrap() + tile.to_digit(10).unwrap() as i32;
        let node = Node {
            x,
            y,
            dir,
            straight,
        };
        if nodes.contains_key(&node) {
            nodes.entry(node).and_modify(|h| *h = *h.min(&mut heat));
        } else {
            nodes.insert(node.clone(), heat);
            unseen.insert(node);
        }
    } else {
        return;
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = Map2D::from_string(
            "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
                .to_string(),
        );
        assert_eq!(pt1(&input, false), 94);

        let input = Map2D::from_string(
            "111111111111
999999999991
999999999991
999999999991
999999999991"
                .to_string(),
        );
        assert_eq!(pt1(&input, false), 71);
    }
}
