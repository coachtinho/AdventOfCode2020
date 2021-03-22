use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challenge1
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: HashMap<usize, usize> = HashMap::new();
    for line in contents.lines() {
        let mut iter = line.split(" ");
        let command = iter.next().unwrap();
        iter.next();
        let arg = iter.next().unwrap();

        if command.starts_with("mask") {
            mask = read_mask(arg); 
        } else if command.starts_with("mem") {
            let i: usize = command
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
            let arg: usize = arg.parse().unwrap();
            let value = apply_mask1(&mask, arg);
            mem.insert(i, value);
        }
    }
    println!("Challenge1: {}", mem.values().sum::<usize>());
        
    // Challenge2
    let mut mem: HashMap<usize, usize> = HashMap::new();
    let mut mask: HashMap<usize, usize> = HashMap::new();
    for line in contents.lines() {
        let mut iter = line.split(" ");
        let command = iter.next().unwrap();
        iter.next();
        let arg = iter.next().unwrap();

        if command.starts_with("mask") {
            mask = read_mask(arg); 
        } else if command.starts_with("mem") {
            let i: usize = command
                .split("[")
                .nth(1)
                .unwrap()
                .split("]")
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
            let arg: usize = arg.parse().unwrap();
            let addresses = apply_mask2(&mask, i);
            for address in addresses.iter() {
                mem.insert(*address, arg);
            }
        }
    }
    println!("Challenge2: {}", mem.values().sum::<usize>());
}

fn apply_mask2(mask: &HashMap<usize, usize>, index: usize) -> Vec<usize> {
    let mut addresses = vec![index];

    for bit in 0..36 {
        match mask.get(&bit) {
            Some(1) => {
                for i in 0..addresses.len() {
                    if addresses[i] % (2_usize.pow(bit as u32 + 1)) < 2_usize.pow(bit as u32) {
                        addresses[i] += 2_usize.pow(bit as u32)
                    }
                }
            },
            None => {
                for i in 0..addresses.len() {
                    if addresses[i] % (2_usize.pow(bit as u32 + 1)) < 2_usize.pow(bit as u32) {
                        addresses.push(addresses[i] + 2_usize.pow(bit as u32));
                    } else {
                        addresses.push(addresses[i] - 2_usize.pow(bit as u32));
                    }
                }
            },
            _ => continue,
        };
    }

    addresses
}

fn apply_mask1(mask: &HashMap<usize, usize>, mut arg: usize) -> usize {
    for bit in mask.keys() {
        match mask.get(bit).unwrap() {
            1 => {
                if arg % (2_usize.pow(*bit as u32 + 1)) < 2_usize.pow(*bit as u32) {
                    arg += 2_usize.pow(*bit as u32);
                }
            },
            0 => {
                if arg % (2_usize.pow(*bit as u32 + 1)) >= 2_usize.pow(*bit as u32) {
                    arg -= 2_usize.pow(*bit as u32);
                }
            },
            _ => continue,
        };
    }

    arg
}

fn read_mask(bits: &str) -> HashMap<usize, usize> {
    let mut mask: HashMap<usize, usize> = HashMap::new();
    for (i, c) in bits.chars().enumerate() {
        match c {
            '1' => mask.insert(35 - i, 1),
            '0' => mask.insert(35 - i, 0),
            _ => continue,
        };
    } 

    mask
}
