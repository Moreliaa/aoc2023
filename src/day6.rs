pub fn run() {
    let times = [48, 93, 85, 95];
    let dist = [296, 1928, 1236, 1391];
    println!("Day6 Pt1: {}", pt1_2(&times, &dist));
    let times = [48938595];
    let dist = [296192812361391];
    println!("Day6 Pt2: {}", pt1_2(&times, &dist));
}

fn pt1_2(times: &[i128], dist: &[i128]) -> i128 {
    let mut result = 1;
    for i in 0..times.len() {
        result *= count_wins(times[i], dist[i]);
    }
    result
}

fn count_wins(time: i128, record: i128) -> i128 {
    let mut count = 0;
    for i in 0..=time {
        if i * (time - i) > record {
            count += 1;
        }
    }
    count
}
