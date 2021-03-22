use std::fs;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challenge1
    let count1: usize = contents
        .split("\n\n")
        .map(|x| calc_group1(x))
        .sum();
    println!("Challenge1: {}", count1);

    // Challenge2
    let count2: usize = contents
        .split("\n\n")
        .map(|x| calc_group2(x))
        .sum();
    println!("Challenge2: {}", count2);
}

fn calc_group1(group: &str) -> usize {
    let mut counted = String::new();

    for c in group.chars() {
        if c == '\n' {
            continue;
        }

        if !counted.contains(c) {
            counted.push(c);
        } 
    }

    counted.len()
}

fn calc_group2(group: &str) -> usize {
    let mut counted = String::new();

    for person in group.lines() {
        for c in person.chars() {
            let mut consistent = true;
            for person2 in group.lines().clone() {
                if !person2.contains(c) {
                    consistent = false;
                    break;
                }
            }
            if consistent && !counted.contains(c) {
                counted.push(c);
            }
        }
    }

    counted.len()
}
