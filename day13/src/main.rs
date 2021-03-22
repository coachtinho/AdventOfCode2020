use std::fs;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let mut iter = contents.lines();
    let estimate: isize = iter.next().unwrap().parse().unwrap();
    let ids: Vec<&str> = iter
        .next()
        .unwrap()
        .split(",")
        .collect();

    // Challenge1
    let mut result = -1;
    let mut new_estimate = estimate;
    while result == -1 {
        for id in ids.iter() {
            if *id == "x" {
                continue;
            }

            let id: isize = id.parse().unwrap();
            if new_estimate % id == 0 {
                result = id * (new_estimate - estimate); 
                break;
            }
        }
        new_estimate += 1;
    }
    println!("Challenge1: {}", result);

    // Challege2
    let mut timestamp = 0;
    let mut step: usize = 1;

    for (i, id) in ids.iter().enumerate() {
        if *id == "x" {
            continue;
        }

        let id: isize = id.parse().unwrap();
        for t in (timestamp..isize::MAX).step_by(step) {
            if (t + i as isize) % id == 0 {
                timestamp = t;
                step *= id as usize;
                break;
            }
        }
    }
    println!("Challenge2: {}", timestamp);
}
