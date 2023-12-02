use fancy_regex::Regex;

pub fn run(input: String) {
    println!("Day2 Pt1: {}", pt1(&input));
    println!("Day2 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i32 {
    let rx_id = Regex::new(r"^Game (\d+):").unwrap();
    let rx_pulls = Regex::new(r"(?=(\d+) ([a-z]+)[,;\n]?\s?)").unwrap();
    let mut ids = 0;
    let max = (12, 13, 14);
    'lines: for l in input.lines() {
        let cap_id = rx_id.captures(l).unwrap().unwrap();
        let id = cap_id.get(1).unwrap().as_str().parse::<i32>().unwrap();
        for c in rx_pulls.captures_iter(l) {
            let c = c.unwrap();
            let count = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = c.get(2).unwrap().as_str();
            if count > match color {
                "red" => max.0,
                "green" => max.1,
                "blue" => max.2,
                _ => panic!(),
            } {
                continue 'lines;
            }
        }
        ids += id;
    }
    ids
}

fn pt2(input: &String) -> i32 {
    let rx_pulls = Regex::new(r"(?=(\d+) ([a-z]+)[,;\n]?\s?)").unwrap();
    let mut powers = 0;
    for l in input.lines() {
        let mut max = (0,0,0);
        for c in rx_pulls.captures_iter(l) {
            let c = c.unwrap();
            let count = c.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let color = c.get(2).unwrap().as_str();
            match color {
                "red" => max.0 = max.0.max(count),
                "green" => max.1 = max.1.max(count),
                "blue" => max.2 = max.2.max(count),
                _ => panic!(),
            }
        }
        powers += max.0 * max.1 * max.2;
    }
    powers
}
