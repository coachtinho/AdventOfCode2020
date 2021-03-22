use std::fs;

fn main() {
    // Read input
    let input = fs::read_to_string("sortedInput.txt").expect("Failed reading file");
    let lines: Vec<isize> = input
        .lines()
        .map(|x| x.parse().expect("Failed parsing line"))
        .collect();

    // Challenge 1
    let mut count = 0;
    for i in (0..lines.len()).rev() {
        let target = 2020 - lines[i];
        let mut found = false;
        for j in 0..lines.len() {
            count += 1;
            if lines[j] == target {
                println!("{}: {}", count, lines[i] * lines[j]);
                found = true;
                break;
            } else if lines[j] > target {
                break;
            }
        }
        if found {
            break;
        }
    } 

    // Challenge 2
    count = 0;
    for i in 0..lines.len() {
        let target1 = 2020 - lines[i];
        for j in i+1..lines.len() {
            let target2 = target1 - lines[j];
            for k in j+1..lines.len() {
                count += 1;
                if lines[k] == target2 {
                    println!("{}: {}", count, lines[i] * lines[j] * lines[k])
                }
            }
        }
    }
}
