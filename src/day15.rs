use itertools::Itertools;

pub fn run(input: String) {
    println!("Day15 Pt1: {}", pt1(&input));
    println!("Day15 Pt2: {}", pt2(&input));
}

fn pt1(input: &String) -> i128 {
    let mut sum = 0;

    for s in input.split(',') {
        sum += hash(s);
    }
    sum
}

fn hash(line: &str) -> i128 {
    let mut current_value = 0;
    for c in line.chars() {
        match c {
            ',' => panic!(),
            '\n' => (),
            _ => {
                let ascii_code = c as u8;
                current_value += ascii_code as i128;
                current_value *= 17;
                current_value = current_value % 256;
            }
        }
    }
    current_value
}

fn pt2(input: &String) -> i128 {
    let mut boxes: Vec<Vec<(&str, i128)>> = vec![vec![]; 256];

    for s in input.split(',') {
        let is_dash = s.contains('-');
        let label = if is_dash {
            &s[0..&s.len() - 1]
        } else {
            &s.split('=').next().unwrap()
        };
        let box_idx = hash(label) as usize;

        let boxx = boxes.get_mut(box_idx).unwrap();
        let mut i = boxx.len();

        if is_dash {
            while i > 0 {
                i -= 1;
                if boxx[i].0 == label {
                    boxx.remove(i);
                }
            }
        } else {
            let focal_len = s.trim().split('=').last().unwrap().parse::<i128>().unwrap();
            let mut is_found = false;
            while i > 0 {
                i -= 1;
                if boxx[i].0 == label {
                    is_found = true;
                    boxx.splice(i..i + 1, [(label, focal_len)]);
                }
            }
            if !is_found {
                boxx.push((label, focal_len));
            }
        }
    }

    let mut result = 0;
    for (box_idx, b) in boxes.into_iter().enumerate() {
        let box_number = box_idx as i128;
        for (lens_idx, l) in b.into_iter().enumerate() {
            let slot = lens_idx as i128 + 1;
            let focusing_power = (1 + box_number) * slot * l.1;
            result += focusing_power;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".to_string();
        assert_eq!(pt1(&input), 1320);
        assert_eq!(pt2(&input), 145);
    }
}
