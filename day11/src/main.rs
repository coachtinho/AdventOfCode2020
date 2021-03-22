use std::fs;

const DIRS: [(isize, isize);8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let mut seats: Vec<Vec<char>> = contents
        .lines()
        .map(|x| x.chars().collect())
        .collect();

    // Challenge1
    while do_round1(&mut seats) {}
    let count1: usize = seats
        .iter()
        .map(|x| {
            x.iter().filter(|y| **y == '#').count()
        })
        .sum();
    println!("Challenge1: {}", count1);

    // Challenge2
    let mut seats: Vec<Vec<char>> = contents
        .lines()
        .map(|x| x.chars().collect())
        .collect();
    while do_round2(&mut seats) {}
    let count2: usize = seats
        .iter()
        .map(|x| {
            x.iter().filter(|y| **y == '#').count()
        })
        .sum();
    println!("Challenge2: {}", count2);
}

fn print_seats(seats: &Vec<Vec<char>>) {
    for x in 0..seats[0].len() {
        for y in 0..seats.len() {
            print!("{} ", seats[x][y]);
        }
        print!("\n");
    }
    print!("\n");
}

fn do_round2(seats: &mut Vec<Vec<char>>) -> bool {
    let mut result = false;
    let old_seats = seats.clone();

    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            let mut occupied = 0;

            if old_seats[y][x] == '.' {
                continue;
            }

            for dir in DIRS.iter() {
                if check_dir(&old_seats, &(x, y), dir) {
                    occupied += 1;
                }
            }

            if seats[y][x] == 'L' && occupied == 0 {
                seats[y][x] = '#';
                result = true;
            } else if seats[y][x] == '#' && occupied >= 5 {
                seats[y][x] = 'L';
                result = true;
            }
        }
    }

    result
}

fn check_dir(seats: &Vec<Vec<char>>, current: &(usize, usize), dir: &(isize, isize)) -> bool {
    let mut newx = current.0 as isize + dir.0;
    let mut newy = current.1 as isize + dir.1;

    while in_bounds((newx, newy), seats) && seats[newy as usize][newx as usize] == '.' {
        newx += dir.0; 
        newy += dir.1; 
    }

    if in_bounds((newx, newy), seats) {
        seats[newy as usize][newx as usize] == '#'
    } else {
        false
    }
}

fn do_round1(seats: &mut Vec<Vec<char>>) -> bool {
    let mut result = false;
    let old_seats = seats.clone();

    for y in 0..seats.len() {
        for x in 0..seats[y].len() {
            let mut occupied = 0;

            if old_seats[y][x] == '.' {
                continue;
            }

            for (dx, dy) in DIRS.iter() {
                let newx: isize = x as isize + *dx;
                let newy: isize = y as isize + *dy;
                if in_bounds((newx, newy), &seats)
                    && old_seats[newy as usize][newx as usize] == '#' {

                    occupied += 1;
                }
            }

            if seats[y][x] == 'L' && occupied == 0 {
                seats[y][x] = '#';
                result = true;
            } else if seats[y][x] == '#' && occupied >= 4 {
                seats[y][x] = 'L';
                result = true;
            }
        }
    }

    result
}

fn in_bounds(seat: (isize, isize), seats: &Vec<Vec<char>>) -> bool {
    seat.0 >= 0 && seat.0 < seats[0].len() as isize && seat.1 >= 0 && seat.1 < seats.len() as isize
}
