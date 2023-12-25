use itertools::Itertools;

pub fn run(input: String) {
    println!("Day24 Pt1: {}", pt1(&input, 200000000000000.0, 400000000000000.0));
    println!("Day24 Pt2: {}", pt2(&input));
}

fn pt1(input: &String, range_min: f64, range_max: f64) -> i128 {
    let input = input.lines().map(|l| l.split(" @ ").map(|s| s.split(", ").map(|n| n.parse::<i128>().unwrap()).collect_tuple::<(i128,i128,i128)>().unwrap()).collect_tuple::<((i128,i128,i128),(i128,i128,i128))>().unwrap()).collect_vec();

    let mut result = 0;
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() {
            let c1 = input[i].0;
            let c2 = input[j].0;
            let m1 = input[i].1;
            let m2 = input[j].1;
            if let Some((x, y)) = check_intersection(c1.0, c1.1, c1.0 + m1.0, c1.1 + m1.1, c2.0, c2.1, c2.0 + m2.0, c2.1 + m2.1) {
                if range_min <= x && x <= range_max && range_min <= y && y <= range_max {
                    result += 1;
                }
            }
        }
    }
    result // 22371 too high
}

fn check_intersection(line1_x0:i128, line1_y0:i128, line1_x1:i128, line1_y1:i128, line2_x0:i128, line2_y0:i128, line2_x1:i128, line2_y1:i128) -> Option<(f64,f64)> {
    let denominator = ((line2_y1 - line2_y0) * (line1_x1 - line1_x0)) - ((line2_x1 - line2_x0) * (line1_y1 - line1_y0));
    if denominator == 0 {
        println!("{} {}, {} {}", line1_x1 - line1_x0, line1_y1 - line1_y0, line2_x1 - line2_x0, line2_y1 - line2_y0);
        return None;
    }
    let line1_x0 = line1_x0 as f64;
    let line1_x1 = line1_x1 as f64;
    let line1_y0 = line1_y0 as f64;
    let line1_y1 = line1_y1 as f64;

    let line2_x0 = line2_x0 as f64;
    let line2_x1 = line2_x1 as f64;
    let line2_y0 = line2_y0 as f64;
    let line2_y1 = line2_y1 as f64;

    let denominator = denominator as f64;

    let mut a = line1_y0 - line2_y0;
    let mut b = line1_x0 - line2_x0;
    let numerator1 = ((line2_x1 - line2_x0) * a) - ((line2_y1 - line2_y0) * b);
    let numerator2 = ((line1_x1 - line1_x0) * a) - ((line1_y1 - line1_y0) * b);
    a = numerator1 / denominator;
    b = numerator2 / denominator;

    let x = line1_x0 + (a * (line1_x1 - line1_x0));
    let y = line1_y0 + (a * (line1_y1 - line1_y0));
    
    let line1_delta_y = line1_y1 - line1_y0;
    if (line1_delta_y > 0.0 && y < line1_y0) || (line1_delta_y < 0.0 && y > line1_y0) {
        return None;
    }

    let line2_delta_y = line2_y1 - line2_y0;
    if (line2_delta_y > 0.0 && y < line2_y0) || (line2_delta_y < 0.0 && y > line2_y0) {
        return None;
    }

    Some((x, y))
}

fn pt2(input: &String) -> i128 {
    0
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;
    #[test]
    fn test() {
        let input = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3".to_string();
        assert_eq!(pt1(&input, 7.0, 27.0), 2);
    }
}
