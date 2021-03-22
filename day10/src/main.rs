use std::fs;
use std::collections::{HashSet, HashMap};

fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let mut numbers: Vec<isize> = contents
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();

    numbers.sort();
    numbers.insert(0, 0);
    numbers.push(numbers.last().unwrap() + 3);

    // Challenge1
    let mut count1 = 0;
    let mut count3 = 0;
    for (n1, n2) in numbers.iter().clone().zip(numbers.iter().skip(1)) {
        if n2 - n1 == 1 {
            count1 += 1;
        } else if n2 - n1 == 3 {
            count3 += 1;
        }
    }
    println!("Challenge1: {}", count1 * count3);

    // Challenge2
    let numbers: HashSet<isize> = contents
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect();
    let count = count_arrangements(&numbers, *numbers.iter().max().unwrap(), &mut HashMap::new());
    println!("Challenge2: {}", count);
}

fn count_arrangements(adapters: &HashSet<isize>, value: isize, memo: &mut HashMap<isize, isize>) -> isize {
    if let Some(&ret) = memo.get(&value) {
        return ret;
    };
    if value == 0 {
        memo.insert(value, 1);
        return 1;
    }
    if value < 0 || !adapters.contains(&value) {
        memo.insert(value, 0);
        return 0;
    };
    let result = count_arrangements(adapters, value - 1, memo) +
                 count_arrangements(adapters, value - 2, memo) +
                 count_arrangements(adapters, value - 3, memo);
    memo.insert(value, result);
    return result;
}
