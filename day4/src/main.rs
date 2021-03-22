mod passport;

use std::fs;
use passport::Passport;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challenge1
    let passports: Vec<Passport> = contents
        .split("\n\n")
        .map(|x| Passport::new(x).unwrap())
        .collect();
    println!("Challenge1: {}", passports.iter().filter(|x| x.is_complete()).count());

    // Challenge2
    let passports: Vec<Passport> = contents
        .split("\n\n")
        .map(|x| Passport::new(x).unwrap())
        .collect();
    println!("Challenge2: {}", passports.iter().filter(|x| x.is_valid()).count());
}
