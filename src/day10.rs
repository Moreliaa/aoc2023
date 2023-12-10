use aoc_lib::map2d::Map2D;
use std::collections::HashSet;

pub fn run(input: String) {
    let mut map = Map2D::from_string(input);
    let pt1 = pt1(&mut map, 'L');
    println!("Day10 Pt1: {}", pt1.0);
    println!("Day10 Pt2: {}", pt2(&mut map, pt1.1));
}

fn pt1(input: &mut Map2D<char>, start_pipe: char) -> (i32, Vec<(i32, i32)>) {
    let (x_start, y_start) = starting_pos(input, start_pipe);
    let (mut x, mut y) = (x_start, y_start);
    let (mut x_last, mut y_last) = (x_start, y_start);
    let mut steps = 0;
    let mut coords = vec![];
    while steps == 0 || (x != x_start || y != y_start) {
        coords.push((x, y));
        let next = step(input, x, y, x_last, y_last);
        x_last = x;
        y_last = y;
        x = next.0;
        y = next.1;
        steps += 1;
    }
    if steps % 2 != 0 {
        panic!("{steps}");
    }
    (steps / 2, coords)
}

fn step(input: &Map2D<char>, x: i32, y: i32, x_last: i32, y_last: i32) -> (i32, i32) {
    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let from_pipe = *input.get(x, y).unwrap();
    let from = ['S', 'N', 'E', 'W'];
    let mut result = vec![];
    for i in 0..dir.len() {
        let d = dir[i];
        let f = from[i];
        let x1 = x + d.0;
        let y1 = y + d.1;
        if x1 == x_last && y1 == y_last {
            continue;
        }
        let ok = match f {
            'S' => from_pipe == '|' || from_pipe == 'L' || from_pipe == 'J',
            'N' => from_pipe == '|' || from_pipe == '7' || from_pipe == 'F',
            'E' => from_pipe == '-' || from_pipe == 'J' || from_pipe == '7',
            'W' => from_pipe == '-' || from_pipe == 'L' || from_pipe == 'F',
            _ => panic!(),
        };
        if !ok {
            continue;
        }
        if let Some(pipe) = input.get(x1, y1) {
            let pipe = *pipe;
            let ok = match f {
                'N' => pipe == '|' || pipe == 'L' || pipe == 'J',
                'S' => pipe == '|' || pipe == '7' || pipe == 'F',
                'W' => pipe == '-' || pipe == 'J' || pipe == '7',
                'E' => pipe == '-' || pipe == 'L' || pipe == 'F',
                _ => panic!(),
            };
            if ok {
                result.push((x1, y1));
            }
        }
    }
    result[0]
}

fn starting_pos(input: &mut Map2D<char>, start_pipe: char) -> (i32, i32) {
    for y in 0..input.height() {
        for x in 0..input.width() {
            if *input.get(x, y).unwrap() == 'S' {
                input.set(x, y, start_pipe);
                return (x, y);
            }
        }
    }
    panic!();
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Side {
    Any,
    Left,
    Right,
    Top,
    Bottom,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(PartialEq, Eq, Hash)]
struct Node {
    x: i32,
    y: i32,
    side: Side,
}

fn pt2(input: &mut Map2D<char>, coords: Vec<(i32, i32)>) -> i32 {
    for y in 0..input.height() {
        'line: for x in 0..input.width() {
            for (x1, y1) in coords.iter() {
                if x == *x1 && y == *y1 {
                    continue 'line;
                }
            }
            input.set(x, y, '.');
        }
    }

    let mut seen: HashSet<Node> = HashSet::new();
    let mut todos: Vec<Node> = vec![Node {
        x: 0,
        y: 0,
        side: Side::Any,
    }];
    while todos.len() > 0 {
        let current_node = todos.pop().unwrap();
        if seen.contains(&current_node) {
            continue;
        }

        let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let from_pipe = *input.get(current_node.x, current_node.y).unwrap();
        let from = ['S', 'N', 'E', 'W'];
        for i in 0..dir.len() {
            let d = dir[i];
            let f = from[i];
            let x1 = current_node.x + d.0;
            let y1 = current_node.y + d.1;

            let ok = match from_pipe {
                '.' => {
                    input.set(current_node.x, current_node.y, 'O');
                    true
                }
                'O' => panic!(),
                '|' => match f {
                    'S' | 'N' => true,
                    'E' if current_node.side == Side::Right => false,
                    'E' => true,
                    'W' if current_node.side == Side::Left => false,
                    'W' => true,
                    _ => panic!(),
                },
                '-' => match f {
                    'W' | 'E' => true,
                    'N' if current_node.side == Side::Top => false,
                    'N' => true,
                    'S' if current_node.side == Side::Bottom => false,
                    'S' => true,
                    _ => panic!(),
                },
                'L' => match f {
                    'E' | 'N' if current_node.side == Side::TopRight => false,
                    _ => true,
                },
                'J' => match f {
                    'W' | 'N' if current_node.side == Side::TopLeft => false,
                    _ => true,
                },
                '7' => match f {
                    'W' | 'S' if current_node.side == Side::BottomLeft => false,
                    _ => true,
                },
                'F' => match f {
                    'E' | 'S' if current_node.side == Side::BottomRight => false,
                    _ => true,
                },
                _ => panic!(),
            };
            if !ok {
                continue;
            }
            if let Some(next_tile) = input.get(x1, y1) {
                let pipe = *next_tile;
                let mut next_node = Node {
                    x: x1,
                    y: y1,
                    side: Side::Any,
                };

                match pipe {
                    '.' => (),
                    'O' => continue,
                    '|' => {
                        next_node.side = match f {
                            'E' => Side::Right,
                            'W' => Side::Left,
                            _ => {
                                if current_node.side == Side::BottomLeft
                                    || current_node.side == Side::Left
                                    || current_node.side == Side::TopLeft
                                {
                                    Side::Left
                                } else {
                                    Side::Right
                                }
                            }
                        };
                    }
                    '-' => {
                        next_node.side = match f {
                            'N' => Side::Top,
                            'S' => Side::Bottom,
                            _ => {
                                if current_node.side == Side::Top
                                    || current_node.side == Side::TopLeft
                                    || current_node.side == Side::TopRight
                                {
                                    Side::Top
                                } else {
                                    Side::Bottom
                                }
                            }
                        };
                    }
                    'L' => {
                        next_node.side = match f {
                            'N' => {
                                if current_node.side == Side::BottomLeft
                                    || current_node.side == Side::Left
                                    || current_node.side == Side::TopLeft
                                {
                                    Side::BottomLeft
                                } else {
                                    Side::TopRight
                                }
                            }
                            'S' => Side::BottomLeft,
                            'W' => Side::BottomLeft,
                            'E' => {
                                if current_node.side == Side::Top
                                    || current_node.side == Side::TopLeft
                                    || current_node.side == Side::TopRight
                                {
                                    Side::TopRight
                                } else {
                                    Side::BottomLeft
                                }
                            }
                            _ => panic!(),
                        };
                    }
                    'J' => {
                        next_node.side = match f {
                            'N' => {
                                if current_node.side == Side::BottomLeft
                                    || current_node.side == Side::Left
                                    || current_node.side == Side::TopLeft
                                {
                                    Side::TopLeft
                                } else {
                                    Side::BottomRight
                                }
                            }
                            'S' => Side::BottomRight,
                            'E' => Side::BottomRight,
                            'W' => {
                                if current_node.side == Side::Top
                                    || current_node.side == Side::TopLeft
                                    || current_node.side == Side::TopRight
                                {
                                    Side::TopLeft
                                } else {
                                    Side::BottomRight
                                }
                            }
                            _ => panic!(),
                        };
                    }
                    '7' => {
                        next_node.side = match f {
                            'N' => Side::TopRight,
                            'E' => Side::TopRight,
                            'S' => {
                                if current_node.side == Side::Left
                                    || current_node.side == Side::BottomLeft
                                    || current_node.side == Side::TopLeft
                                {
                                    Side::BottomLeft
                                } else {
                                    Side::TopRight
                                }
                            }
                            'W' => {
                                if current_node.side == Side::Top
                                    || current_node.side == Side::TopLeft
                                    || current_node.side == Side::TopRight
                                {
                                    Side::TopRight
                                } else {
                                    Side::BottomLeft
                                }
                            }
                            _ => panic!(),
                        };
                    }
                    'F' => {
                        next_node.side = match f {
                            'N' => Side::TopLeft,
                            'W' => Side::TopLeft,
                            'S' => {
                                if current_node.side == Side::Left
                                    || current_node.side == Side::BottomLeft
                                    || current_node.side == Side::TopLeft
                                {
                                    Side::TopLeft
                                } else {
                                    Side::BottomRight
                                }
                            }
                            'E' => {
                                if current_node.side == Side::Top
                                    || current_node.side == Side::TopLeft
                                    || current_node.side == Side::TopRight
                                {
                                    Side::TopLeft
                                } else {
                                    Side::BottomRight
                                }
                            }
                            _ => panic!(),
                        };
                    }
                    _ => panic!("{pipe}"),
                }
                if !seen.contains(&next_node) && !todos.contains(&next_node) {
                    todos.push(next_node);
                }
            }
        }

        seen.insert(current_node);
    }
    input.aggregate(|tile, _, _| if *tile == '.' { 1 } else { 0 })
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "...........
.F-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
            .to_string();
        let mut map = Map2D::from_string(input);
        println!("Day10 Pt2: {}", pt2(&mut map, vec![]));
        map.print();

        let input = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            .to_string();
        let mut map = Map2D::from_string(input);
        let pt1 = pt1(&mut map, '7');
        println!("Day10 Pt2: {}", pt2(&mut map, pt1.1));
        map.print();
    }
}
