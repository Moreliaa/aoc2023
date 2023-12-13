use aoc_lib::map2d::Map2D;

pub fn run(input: String) {
    let map = Map2D::from_string(input);
    println!("Day11 Pt1: {}", pt1_2(&map, 1));
    println!("Day11 Pt2: {}", pt1_2(&map, 1000000 - 1));
}

fn pt1_2(input: &Map2D<char>, expansion: i128) -> i128 {
    let (stars, rows, cols) = read_map(input);
    let mut sum = 0;
    for i in 0..stars.len() {
        let s1 = stars[i];
        for j in i + 1..stars.len() {
            let s2 = stars[j];
            let rows_to_check = [s2.0.min(s1.0), s2.0.max(s1.0)];
            let cols_to_check = [s2.1.min(s1.1), s2.1.max(s1.1)];
            let distance_row_expansion = rows.iter().fold(0, |acc, r| {
                if rows_to_check[0] < *r && *r < rows_to_check[1] {
                    acc + expansion
                } else {
                    acc
                }
            });
            let distance_col_expansion = cols.iter().fold(0, |acc, c| {
                if cols_to_check[0] < *c && *c < cols_to_check[1] {
                    acc + expansion
                } else {
                    acc
                }
            });
            sum += rows_to_check[1] - rows_to_check[0] + cols_to_check[1] - cols_to_check[0]
                + distance_row_expansion
                + distance_col_expansion;
        }
    }
    sum
}

fn read_map(input: &Map2D<char>) -> (Vec<(i128, i128)>, Vec<i128>, Vec<i128>) {
    let mut stars = vec![];
    let mut rows = vec![];
    let mut cols = vec![];
    for row in 0..input.height() as i128 {
        let mut stars_count = 0;
        for col in 0..input.width() as i128 {
            if *input.get(row as i32, col as i32).unwrap() == '#' {
                stars.push((row, col));
                stars_count += 1;
            }
        }
        if stars_count == 0 {
            rows.push(row);
        }
    }

    'cols: for col in 0..input.width() as i128 {
        for row in 0..input.height() as i128 {
            if *input.get(row as i32, col as i32).unwrap() == '#' {
                continue 'cols;
            }
        }
        cols.push(col);
    }

    (stars, rows, cols)
}
