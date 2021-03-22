use std::collections::HashMap;

fn main() {
    let input = vec![16, 11, 15, 0, 1, 7];

    // Challenge1
    let mut mem: HashMap<usize, Vec<usize>> = HashMap::new();
    for (i, n) in input.iter().enumerate() {
        let mut v: Vec<usize> = Vec::new();
        v.push(i + 1);
        mem.insert(*n, v);
    }

    let mut spoken = input[input.len() - 1];
    let mut round = input.len() + 1;
    while round <= 30000000 {
        if mem.get(&spoken).unwrap().len() < 2 {
            spoken = 0;
        } else {
            let v = mem.get(&spoken).unwrap();
            spoken = v[v.len() - 1] - v[v.len() - 2]; 
        }

        let v = mem.entry(spoken).or_insert(Vec::new());
        v.push(round);

        round += 1;
    }
    println!("Challenge1: {}", spoken);
}
