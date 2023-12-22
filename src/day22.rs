use aoc_lib::map2d::Map2D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    println!("Day22 Pt1: {}", pt1(&input, 10));
    println!("Day22 Pt2: {}", pt2(&input));
}

const FREE: i32 = 0;
const FLOOR: i32 = 1;

fn pt1(input: &String, size: i32) -> i32 {
    let mut layers: Vec<Map2D<i32>> = vec![Map2D::new(size, size, FLOOR)];
    let mut block_id = 1;
    let mut blocks:HashMap<i32, ((i32,i32,i32),(i32,i32,i32))> = HashMap::new();
    
    for l in input.lines() {
        block_id+=1;
        let ((x1, y1, z1), (x2, y2, z2)) = l.split('~').map(|c| c.split(',').map(|a| a.parse::<i32>().unwrap()).collect_tuple().unwrap()).collect_tuple().unwrap();
        blocks.insert(block_id, ((x1,y1,z1), (x2,y2,z2)));
        // snapshot
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    while layers.len() <= z as usize {
                        layers.push(Map2D::new(size, size, FREE));
                    }
                    layers[z as usize].set(x, y, block_id);
                }
            }
        }
    }

    // drops
    let mut seen_blocks: HashSet<i32> = HashSet::new();
    for z in 1..layers.len() {
        for x in 0..layers[z].width() {
            for y in 0..layers[z].height() {
                let block_id = *layers[z].get(x,y).unwrap();
                if block_id != FREE && !seen_blocks.contains(&block_id) {
                    // drop
                    seen_blocks.insert(block_id);
                    let ((x1, y1, z1), (x2, y2, z2)) = blocks.get(&block_id).unwrap();

                    // calc drop distance
                    let mut drop_distance = 1;
                    'drop: loop {
                        for x_brick in *x1..=*x2 {
                            for y_brick in *y1..=*y2 {
                                if *layers[z-drop_distance].get(x_brick,y_brick).unwrap() != FREE {
                                    break 'drop;
                                }
                            }
                        }
                        drop_distance += 1;
                    }
                    drop_distance -= 1;

                    if drop_distance == 0 {
                        continue;
                    }

                    // delete old brick
                    for x_brick in *x1..=*x2 {
                        for y_brick in *y1..=*y2 {
                            for z_brick in *z1..=*z2 {
                                layers[z_brick as usize].set(x_brick, y_brick, FREE);
                            }
                        }
                    }

                    // place new brick
                    for x_brick in *x1..=*x2 {
                        for y_brick in *y1..=*y2 {
                            for z_brick in *z1 - drop_distance as i32..=*z2 - drop_distance as i32 {
                                layers[z_brick as usize].set(x_brick, y_brick, block_id);
                            }
                        }
                    }

                    blocks.entry(block_id).and_modify(|e| {e.0.2 -= drop_distance as i32; e.1.2 -= drop_distance as i32;});
                }
            }
        }
    }

    // check supports
    let mut supporting: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut supported_by: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut seen_blocks: HashSet<i32> = HashSet::new();
    for z in 2..layers.len() {
        for x in 0..layers[z].width() {
            for y in 0..layers[z].height() {
                let block_id = *layers[z].get(x,y).unwrap();
                if block_id != FREE && !seen_blocks.contains(&block_id) {
                    seen_blocks.insert(block_id);
                    let ((x1, y1, _), (x2, y2, _)) = blocks.get(&block_id).unwrap();
                    for x_brick in *x1..=*x2 {
                        for y_brick in *y1..=*y2 {
                            let tile = *layers[z-1].get(x_brick,y_brick).unwrap();
                            if tile != FREE {
                                supported_by.entry(block_id).and_modify(|e| {e.insert(tile); }).or_insert({let mut h = HashSet::new(); h.insert(tile); h});
                                supporting.entry(tile).and_modify(|e| { e.insert(block_id);}).or_insert({let mut h = HashSet::new(); h.insert(block_id); h});
                            }
                        }
                    }
                }
            }
        }
    }

    let mut single_supports: HashSet<i32> = HashSet::new();
    for (_, v) in supported_by.iter() {
        if v.len() == 1 {
            single_supports.extend(v);
        }
    }

    /*for z in (0..layers.len()).rev() {
        print!("=={z}==");
        layers[z].print();
        println!();
    }*/

    //println!("Supporting: {:?}", supporting);
    //println!("Supported by: {:?}", supported_by);
    println!("Single supports: {:?}", single_supports);
    (blocks.len() - single_supports.len()) as i32 // 1218 too high // 414 too low
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
        let input = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9".to_string();
        assert_eq!(pt1(&input, 3), 5);
    }
}
