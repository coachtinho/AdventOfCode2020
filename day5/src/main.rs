use std::fs;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challenge1
    let mut ids: Vec<isize> = contents.lines().map(|x| get_id(x)).collect();
    ids.sort();
    println!("Challenge1: {}", ids.last().unwrap());

    // Challenge2
    let iter = ids.iter();
    for (i, j) in iter.clone().zip(iter.skip(1)) {
        if j - i > 1 {
            println!("Challenge2: {}", i + 1);
            break;
        }
    }
}

fn get_id(pass: &str) -> isize {
    let mut min_row: f32 = 0_f32;
    let mut max_row: f32 = 127_f32;
    let mut min_col: f32 = 0_f32;
    let mut max_col: f32 = 7_f32;

    for c in pass.chars() {
        match c {
            'F' => { max_row = (min_row + max_row) / 2_f32; max_row = max_row.floor(); }
            'B' => { min_row = (min_row + max_row) / 2_f32; min_row = min_row.ceil(); }
            'L' => { max_col = (min_col + max_col) / 2_f32; max_col = max_col.floor(); }
            'R' => { min_col = (min_col + max_col) / 2_f32; min_col = min_col.ceil(); }
            _ => {}
        }
    }

    min_row as isize * 8 + min_col as isize
}
