use aoc_lib::map2d::Map2D;
use std::collections::HashSet;

pub fn run(input: String) {
    let input = Map2D::from_string(input);
    println!("Day16 Pt1: {}", pt1(&input, Beam {
        dir: 'E',
        x: 0,
        y: 0
    }));
    println!("Day16 Pt2: {}", pt2(&input));
}


#[derive(PartialEq, Eq, Hash, Clone)]
struct Beam {
    dir: char,
    x: i32,
    y: i32
}

fn pt1(input: &Map2D<char>, initial_beam: Beam) -> i32 {
    let mut beams_seen: HashSet<Beam> = HashSet::new();
    let mut beams_next: Vec<Beam> = vec![];
    beams_next.push(initial_beam);

    while let Some(beam) = beams_next.pop() {
        if beams_seen.contains(&beam) {
            continue;
        }

        let tile = match input.get(beam.x, beam.y) {
            None => continue,
            Some(val) => *val
        };

        match tile {
            '.' => {
                beams_next.push(pass(&beam));
            },
            '|' => {
                match beam.dir {
                    'N' | 'S' => beams_next.push(pass(&beam)),
                    'E' | 'W' => beams_next.extend(split_v(&beam)),
                    _ => panic!()
                }
            },
            '-' => {
                match beam.dir {
                    'E' | 'W' => beams_next.push(pass(&beam)),
                    'N' | 'S' => beams_next.extend(split_h(&beam)),
                    _ => panic!()
                }
            },
            '\\' => {
                let new_dir = match beam.dir {
                    'N' => 'W',
                    'S' => 'E',
                    'W' => 'N',
                    'E' => 'S',
                    _ => panic!()
                };
                let mut new_beam = beam.clone();
                new_beam.dir = new_dir;
                beams_next.push(pass(&new_beam));
            },
            '/' => {
                let new_dir = match beam.dir {
                    'N' => 'E',
                    'S' => 'W',
                    'W' => 'S',
                    'E' => 'N',
                    _ => panic!()
                };
                let mut new_beam = beam.clone();
                new_beam.dir = new_dir;
                beams_next.push(pass(&new_beam));
            },
            _ => panic!(),
        }

        beams_seen.insert(beam.clone());

    }
    input.aggregate(|_, x, y| {
        let mut seen = false;
        for b in beams_seen.iter() {
            if b.x == x && b.y == y {
                seen = true;
                break;
            }
        }
        if seen {
            1
        } else {
            0
        }
    })
}

fn pass(beam: &Beam) -> Beam {
    let mut result = beam.clone();
    let (x, y) = match beam.dir {
        'N' => (0, -1),
        'S' => (0, 1),
        'W' => (-1, 0),
        'E' => (1, 0),
        _ => panic!()
    };
    result.x += x;
    result.y += y;
    result
}

fn split_v(beam: &Beam) -> Vec<Beam> {
    let mut beam_n = beam.clone();
    beam_n.dir = 'N';
    let mut beam_s = beam.clone();
    beam_s.dir = 'S';
    vec![pass(&beam_n), pass(&beam_s)]
}

fn split_h(beam: &Beam) -> Vec<Beam> {
    let mut beam_w = beam.clone();
    beam_w.dir = 'W';
    let mut beam_e = beam.clone();
    beam_e.dir = 'E';
    vec![pass(&beam_w), pass(&beam_e)]
}

fn pt2(input: &Map2D<char>) -> i32 {
    let mut result = 0;
    for x in 0..input.width() {
        println!("x: {x} y: 0");
        result = result.max(pt1(input, Beam {
            dir: 'S',
            x: x,
            y: 0
        }));
        println!("x: {x} y: 1");
        result = result.max(pt1(input, Beam {
            dir: 'N',
            x: x,
            y: input.height() - 1
        }));
    }
    for y in 0..input.height() {
        println!("x: 0 y: {y}");
        result = result.max(pt1(input, Beam {
            dir: 'E',
            x: 0,
            y: y
        }));
        println!("x: 1 y: {y}");
        result = result.max(pt1(input, Beam {
            dir: 'W',
            x: input.width() - 1,
            y: y
        }));
    }
    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        
    }
}
