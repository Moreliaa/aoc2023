use itertools::Itertools;

pub fn run(input: String) {
    println!("Day1 Pt9: {}", pt1_2(&input, true));
    println!("Day1 Pt9: {}", pt1_2(&input, false));
}

fn pt1_2(input: &String, is_pt1: bool) -> i32 {
    let mut sum = 0;
    for l in input.lines() {
        let nums: Vec<i32> = l.split(' ').map(|v| v.parse::<i32>().unwrap()).collect_vec();
        let r = extrapolate(nums);
        sum += if is_pt1 { r.0 } else { r.1 };
    }
    sum
}

fn extrapolate(nums: Vec<i32>) -> (i32, i32) {
    let mut sequences:Vec<Vec<i32>> = vec![nums];
    loop {
        let diffs = diffs(&sequences.last().unwrap());
        let all_zero = diffs.iter().fold(true, |acc, v| acc && *v == 0);
        if all_zero {
            break;
        }
        sequences.push(diffs);
    }
    let (mut next_diff_pt1, mut next_diff_pt2) = (0,0);
    for s in sequences.into_iter().rev() {
        next_diff_pt1 = s.last().unwrap() + next_diff_pt1;
        next_diff_pt2 = s.first().unwrap() - next_diff_pt2;
    }
    (next_diff_pt1, next_diff_pt2)
}

fn diffs(nums: &Vec<i32>) -> Vec<i32> {
    let mut diffs = vec![];
    for (a,b) in nums.iter().tuple_windows() {
        diffs.push(b-a);
    }
    diffs
}
