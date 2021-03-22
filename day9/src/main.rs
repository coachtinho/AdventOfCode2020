use std::fs;

fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let numbers: Vec<isize> = contents
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect();

    // Challenge1
    let mut invalid = -1;
    for (i, n) in numbers.iter().enumerate() {
        if i >= 25 && !is_valid(&numbers, &i) {
            invalid = *n;
            break;
        }
    }
    println!("Challenge1: {}", invalid);

    //Challenge2
    let mut answer: Vec<isize> = Vec::new();
    for i in 0..numbers.len() {
        let mut current = 0;
        let mut j = i;
        answer.clear();
        while current < invalid {
            current += numbers[j];
            j += 1;
            answer.push(numbers[j]);
        }

        if current == invalid {
            break;
        }
    }
    answer.sort();
    println!("Challenge2: {}", answer.first().unwrap() + answer.last().unwrap());
}

fn is_valid(numbers: &Vec<isize>, candidate: &usize) -> bool {
    let target = numbers[*candidate];

    for i in candidate - 25..candidate - 1 {
        for j in i + 1..*candidate {
            if numbers[i] + numbers[j] == target {
                return true;
            }
        }
    }

    false
}
