use aoc_lib::map2d::Map2D;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub fn run(input: String) {
    let (pt1_result, blocks_len, supported_by, supporting) = pt1(&input, 10);
    println!("Day22 Pt1: {}", pt1_result);
    println!("Day22 Pt2: {}", pt2(blocks_len, supported_by, supporting));
}

const FREE: i32 = 0;
const FLOOR: i32 = 1;

fn pt1(
    input: &String,
    size: i32,
) -> (
    i32,
    usize,
    HashMap<i32, HashSet<i32>>,
    HashMap<i32, HashSet<i32>>,
) {
    let mut layers: Vec<Map2D<i32>> = vec![Map2D::new(size, size, FLOOR)];
    let mut block_id = 1;
    let mut blocks: HashMap<i32, ((i32, i32, i32), (i32, i32, i32))> = HashMap::new();

    for l in input.lines() {
        block_id += 1;
        let ((x1, y1, z1), (x2, y2, z2)) = l
            .split('~')
            .map(|c| {
                c.split(',')
                    .map(|a| a.parse::<i32>().unwrap())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_tuple()
            .unwrap();
        blocks.insert(
            block_id,
            (
                (x1.min(x2), y1.min(y2), z1.min(z2)),
                (x2.max(x1), y2.max(y1), z2.max(z1)),
            ),
        );
        // snapshot
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    while layers.len() <= z as usize {
                        layers.push(Map2D::new(size, size, FREE));
                    }
                    if *layers[z as usize].get(x, y).unwrap() != FREE {
                        panic!();
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
                let block_id = *layers[z].get(x, y).unwrap();
                if block_id != FREE && !seen_blocks.contains(&block_id) {
                    // drop
                    seen_blocks.insert(block_id);
                    let ((x1, y1, z1), (x2, y2, z2)) = blocks.get(&block_id).unwrap();

                    // calc drop distance
                    let mut drop_distance = 1;
                    'drop: loop {
                        for x_brick in *x1..=*x2 {
                            for y_brick in *y1..=*y2 {
                                if *layers[z - drop_distance].get(x_brick, y_brick).unwrap() != FREE
                                {
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
                            for z_brick in
                                (*z1 - drop_distance as i32)..=(*z2 - drop_distance as i32)
                            {
                                if *layers[z_brick as usize].get(x_brick, y_brick).unwrap() != FREE
                                {
                                    panic!();
                                }
                                layers[z_brick as usize].set(x_brick, y_brick, block_id);
                            }
                        }
                    }

                    blocks.entry(block_id).and_modify(|e| {
                        e.0 .2 -= drop_distance as i32;
                        e.1 .2 -= drop_distance as i32;
                    });
                }
            }
        }
    }

    // check supports
    let mut supporting: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut supported_by: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut seen_blocks: HashSet<i32> = HashSet::new();
    for z in 1..layers.len() {
        for x in 0..layers[z].width() {
            for y in 0..layers[z].height() {
                let block_id = *layers[z].get(x, y).unwrap();
                if block_id != FREE && !seen_blocks.contains(&block_id) {
                    seen_blocks.insert(block_id);
                    let ((x1, y1, _), (x2, y2, _)) = blocks.get(&block_id).unwrap();
                    for x_brick in *x1..=*x2 {
                        for y_brick in *y1..=*y2 {
                            let tile = *layers[z - 1].get(x_brick, y_brick).unwrap();
                            if tile != FREE && tile != FLOOR {
                                supported_by
                                    .entry(block_id)
                                    .and_modify(|e| {
                                        e.insert(tile);
                                    })
                                    .or_insert({
                                        let mut h = HashSet::new();
                                        h.insert(tile);
                                        h
                                    });
                                supporting
                                    .entry(tile)
                                    .and_modify(|e| {
                                        e.insert(block_id);
                                    })
                                    .or_insert({
                                        let mut h = HashSet::new();
                                        h.insert(block_id);
                                        h
                                    });
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
    (
        (blocks.len() - single_supports.len()) as i32,
        blocks.len(),
        supported_by,
        supporting,
    )
}

fn pt2(
    blocks_len: usize,
    supported_by: HashMap<i32, HashSet<i32>>,
    supporting: HashMap<i32, HashSet<i32>>,
) -> i32 {
    let mut blocks_falling: HashMap<i32, usize> = HashMap::new();
    for idx in 0..blocks_len {
        let block_id = idx as i32 + 2;
        let mut remaining_supported_blocks = supported_by.clone();
        let mut blocks_to_remove = vec![block_id];
        while let Some(block_to_remove) = blocks_to_remove.pop() {
            let supported_blocks = match supporting.get(&block_to_remove) {
                Some(val) => val,
                None => continue,
            };

            for s in supported_blocks {
                remaining_supported_blocks.entry(*s).and_modify(|e| {
                    e.remove(&block_to_remove);
                });
                if remaining_supported_blocks.contains_key(s)
                    && remaining_supported_blocks.get(s).unwrap().len() == 0
                {
                    remaining_supported_blocks.remove(s);
                    blocks_to_remove.push(*s);
                }
            }
        }
        blocks_falling.insert(
            block_id,
            supported_by.len() - remaining_supported_blocks.len(),
        );
    }
    blocks_falling.values().fold(0, |acc, v| acc + v) as i32
}
