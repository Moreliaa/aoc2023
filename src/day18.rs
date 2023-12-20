use fancy_regex::Regex;
use std::collections::HashSet;
use aoc_lib::map2d::Map2D;
use std::cmp::Ordering;

pub fn run(input: String) {
    let lines = parse_input(&input);
    println!("Day18 Pt1: {}", pt1(&lines));
    let lines = parse_input_pt2(&input);
    println!("Day18 Pt2: {}", pt2(&lines));
}

fn parse_input<'a>(input: &'a String) -> Vec<(&'a str, i32)> {
    let re = Regex::new(r"(.) (\d+) \(#(.+)\)").unwrap();
    let mut result = vec![];
    for l in input.lines() {
        let cap = re.captures(l).unwrap().unwrap();
        let dir = cap.get(1).unwrap().as_str();
        let len = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();
        result.push((dir, len));
    }
    result
}

fn parse_input_pt2<'a>(input: &'a String) -> Vec<(&'a str, i32)> {
    let re = Regex::new(r"(.) (\d+) \(#(.+)\)").unwrap();
    let mut result = vec![];
    for l in input.lines() {
        let cap = re.captures(l).unwrap().unwrap();
        let color = cap.get(3).unwrap().as_str();

        let len = i32::from_str_radix(&color[0..5], 16).unwrap();
        let dir = match &color[5..=5] {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => panic!("{}", &color)
        };
        result.push((dir, len));
    }
    result
}

fn pt1(input: &Vec<(&str, i32)>) -> i32 {
    let mut x = 0;
    let mut y = 0;
    let (mut x_min, mut y_min, mut x_max, mut y_max) = (0,0,0,0);
    let mut map: HashSet<(i32,i32)> = HashSet::new();
    map.insert((x,y));

    for (dir, len) in input {
        let mut remaining = *len;
        let coords = match *dir {
            "U" => (0, -1),
            "R" => (1, 0),
            "D" => (0, 1),
            "L" => (-1, 0),
            _ => panic!()
        };

        while remaining > 0 {
            remaining -= 1;
            x += coords.0;
            y += coords.1;
            x_min = x_min.min(x);
            y_min = y_min.min(y);
            x_max = x_max.max(x);
            y_max = y_max.max(y);
            map.insert((x, y));
        }
    }

    let mut map2d = Map2D::new(x_max - x_min + 3, y_max - y_min + 3, '.');
    for coords in map {
        map2d.set(coords.0 - x_min + 1,coords.1 - y_min + 1, '#');
    }

    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    let mut next_nodes = vec![(0,0)];
    let dir = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    while let Some(next) = next_nodes.pop() {
        seen.insert(next.clone());
        if let Some(tile) = map2d.get(next.0, next.1) {
            if *tile == '#' {
                continue;
            }
            map2d.set(next.0,next.1, 'O');
            for d in dir {
                let x = next.0 + d.0;
                let y = next.1 + d.1;
                let t = (x, y);
                if seen.contains(&t) {
                    continue;
                } else {
                    next_nodes.push(t);
                }
            }
        }
        
    }

    map2d.aggregate(|t, _, _| {
        if *t != 'O' { 1 } else { 0 }
    })
}

fn pt2(input: &Vec<(&str, i32)>) -> i128 {
    let (mut x, mut y) = (0,0);
    let mut vertical: Vec<(i32, i32, i32,i32)> = vec![];
    let mut horizontal: Vec<(i32, i32,i32,i32)> = vec![];
    let (mut x_min, mut y_min, mut x_max, mut y_max) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);
    for line in input {
        let x1 = x;
        let y1 = y;
        let x2 = match line.0 {
            "U" => x1,
            "R" => x1 + line.1,
            "D" => x1,
            "L" => x1 - line.1,
            _ => panic!()
        };
        let y2 = match line.0 {
            "U" => y1 - line.1,
            "R" => y1,
            "D" => y1 + line.1,
            "L" => y1,
            _ => panic!()
        };

        x_min = x_min.min(x1).min(x2);
        y_min = y_min.min(y1).min(y2);
        x_max = x_max.max(x1).max(x2);
        y_max = y_max.max(y1).max(y2);

        x = x2;
        y = y2;

        match line.0 {
            "U" => {vertical.push((x2, y2, x1, y1)); }, 
            "D" => {vertical.push((x1, y1, x2, y2)); }, 
            "L" => {horizontal.push((x2, y2, x1, y1)); }, 
            "R" => {horizontal.push((x1, y1, x2, y2));  },
            _ => panic!(),
        };

        
    }
    vertical.sort_by(|a,b| {
        let order = a.0.cmp(&b.0);
        if order == Ordering::Equal {
            a.1.cmp(&b.1)
        } else {
            order
        }
    });
    println!("{:?}", vertical);
    println!("{:?}", horizontal);

    let mut result = 0;
    for y in y_min..=y_max {
        let mut r_v = 0;
        let mut fill = true;
        let mut v = vertical.iter().filter(|a| {
            a.1 <= y && y <= a.3 
        });
        //println!("{y} {:?}", v);
        println!("{y}");
        let mut last = None;
        let mut last_x_filled = None;
        while let Some(v1) = v.next() {
            //println!("{:?}", v1);
            if last == None {
                last = Some(v1);
                continue;
            }
            let offset = if last_x_filled != last { 1 } else { 0 };
            let last_unwrapped = last.unwrap();
            last = Some(v1);
            let h_check1 = (last_unwrapped.0, y, v1.0, y);
            //let h_check2 = (v1.0, y, x_unwrapped, y);
            if horizontal.contains(&h_check1) {
                r_v += v1.0 as i128 - last_unwrapped.0 as i128 + offset;
                last_x_filled = last;
                continue;
            }
            if fill {
                //println!("v1: {} x_un: {x_unwrapped}", v1.0);
                r_v += v1.0 as i128 - last_unwrapped.0 as i128 + offset;
                last_x_filled = last;
            }
            fill = !fill;
        }
        //println!("{r_v} {r_h}");
        result += r_v;
    }

    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)".to_string();
        let lines = parse_input(&input);
        assert_eq!(pt2(&lines), 62);
        let lines = parse_input_pt2(&input);
        println!("{:?}", lines);
        assert_eq!(pt2(&lines), 952408144115);
    }
}
