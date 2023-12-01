pub fn run(input: String) {
    println!("Day1 Pt1: {}", pt1(&input));
    println!("Day1 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> u32 {
    let mut sum = 0;
    for l in input.lines() {
        let mut first = None;
        let mut last = None;
        for c in l.chars() {
            match c {
                '0'..='9' => {
                    if first == None {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
                _ => (),
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    sum
}

fn pt2(input: &String) -> u32 {
    let mut sum = 0;
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    for l in input.lines() {
        let mut first = None;
        let mut last = None;
        println!("{}", l);
        for (i, c) in l.chars().enumerate() {
            match c {
                '0'..='9' => {
                    if first == None {
                        first = c.to_digit(10);
                    }
                    last = c.to_digit(10);
                }
                _ => (),
            }

            for i_n in 0..numbers.len() {
                let n = numbers[i_n];
                if i >= n.len() - 1 {
                    let i_s = i + 1 - n.len();
                    if &l[i_s..=i] == n {
                        if first == None {
                            first = Some(i_n as u32);
                        }
                        last = Some(i_n as u32);
                    }
                }
            }
        }
        sum += first.unwrap() * 10 + last.unwrap();
    }
    sum
}
