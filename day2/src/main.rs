use std::fs;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let lines = contents.lines();

    let mut count1 = 0;
    let mut count2 = 0;
    for line in lines {
        let elements: Vec<&str> = line.split(" ").collect();
        let (n1, n2) = parse_count(elements[0]);
        let chr = elements[1].chars().next().unwrap();

        // Challenge1
        if is_valid1(elements[2], &chr, &n1, &n2) {
            count1 += 1;
        }

        // Challenge2
        if is_valid2(elements[2], &chr, &n1, &n2) {
            count2 += 1;
        }
    }
    println!("Challenge1: {}\nChallenge2: {}", count1, count2);
}


fn parse_count(spec: &str) -> (usize, usize) {
    let  numbers: Vec<usize> = spec
        .split("-")
        .map(|x| x.parse().expect("Failed parsing count"))
        .collect();

    (numbers[0], numbers[1])
}

fn is_valid1(word: &str, chr: &char, min: &usize, max: &usize) -> bool {
    let mut count = 0;
    for c in word.chars() {
        if c == *chr {
            count += 1;
        }
    }

    count >= *min && count <= *max
}

fn is_valid2(word: &str, chr: &char, pos1: &usize, pos2: &usize) -> bool {
    let chars: Vec<char> = word.chars().collect();

    if chars[pos1 - 1] == *chr {
        chars[pos2 - 1] != *chr
    } else {
        chars[pos2 - 1] == *chr
    }
}
