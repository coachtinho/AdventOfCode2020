use std::fs;

struct Vector2D {
    pub x: usize,
    pub y: usize,
}

const VELOCITIES: [Vector2D;5] = [
    Vector2D { x: 1, y: 1 },
    Vector2D { x: 3, y: 1 },
    Vector2D { x: 5, y: 1 },
    Vector2D { x: 7, y: 1 },
    Vector2D { x: 1, y: 2 },
];

fn main() {
    // Read input file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let matrix: Vec<Vec<char>> = contents
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    // Challenge1
    let start = Vector2D { x: 0, y: 0 };
    let velocity = Vector2D { x: 3, y: 1 };
    let count1 = count_trees(&matrix, start, &velocity);
    println!("Challenge1: {}", count1);

    // Challenge2
    let mut count2 = 1;
    for velocity in VELOCITIES.iter() {
        let start = Vector2D { x: 0, y: 0 };
        count2 *= count_trees(&matrix, start, velocity);
    }
    println!("Challenge2: {}", count2);
}

fn count_trees(map: &Vec<Vec<char>>, mut position: Vector2D, velocity: &Vector2D) -> usize {
    let mut count = 0;
    while position.y < map.len() {
        if map[position.y][position.x] == '#' {
            count += 1;
        }

        position.x = (position.x + velocity.x) % map[0].len();
        position.y = position.y + velocity.y;
    }

    count
}
