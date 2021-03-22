use std::fs;
use std::collections::HashMap;
use regex::Regex;

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challege1
    let bag_map = parse_input(&contents);
    let count1 = hold_shinygold(&bag_map);
    println!("Challenge1: {}", count1);

    // Challenge2
    let count2 = bag_count(&bag_map, "shiny gold");
    println!("Challenge2: {}", count2);
}

fn parse_input(input: &str) -> HashMap<String, Vec<(usize, String)>> {
    let mut map = HashMap::new();
    let re = Regex::new(r"(?P<number>[0-9]+) (?P<color>.+?) bag").unwrap();
    for line in input.lines() {
        let mut iter = line.split(" bags contain ");
        let color = iter.next().unwrap().to_string();
        let mut contains: Vec<(usize, String)> = Vec::new();
        for captures in re.captures_iter(line) {
            contains.push(((&captures["number"]).parse().unwrap(), (&captures["color"]).to_string()));
        }
        map.insert(color, contains);
    }
    map
}

fn hold_shinygold(map: &HashMap<String, Vec<(usize, String)>>) -> usize {
    let mut count = 0;
    
    for (color, _) in map.iter() {
        if can_hold(map, color, "shiny gold") {
            count += 1;
        }    
    }

    count
}

fn can_hold(map: &HashMap<String, Vec<(usize, String)>>, holder: &str, target: &str) -> bool {
    let colors = map.get(holder).unwrap();
    
    for (_, color) in colors.iter() {
        if color == target || can_hold(map, color, target) {
            return true;
        }
    }

    false
}

fn bag_count(map: &HashMap<String, Vec<(usize, String)>>, bag: &str) -> usize {
    let mut count = 0;

    let bags = map.get(bag).unwrap();

    for (ammount, color) in bags.iter() {
        count += ammount + ammount * bag_count(map, color); 
    }

    count
}
