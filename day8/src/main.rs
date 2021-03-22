use std::fs;

fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");
    let program: Vec<&str> = contents.lines().collect();

    // Challenge1
    let state = run_program(&program);
    println!("Challenge1: {}", state.1);

    // Challenge2
    for (i, line) in program.iter().enumerate() {
        let mut params = line.split(" ");
        let command = params.next().unwrap();

        match command {
            "nop" => {
                let mut new_prog = program.clone();
                let arg = params.next().unwrap();
                new_prog.remove(i);
                let sub = &format!("jmp {}", arg);
                new_prog.insert(i, sub);
                let state = run_program(&new_prog);

                if state.0 != -1 {
                    println!("Challenge2: {}", state.1);
                    break;
                }
            },
            "jmp" => {
                let mut new_prog = program.clone();
                let arg = params.next().unwrap();
                new_prog.remove(i);
                let sub = &format!("nop {}", arg);
                new_prog.insert(i, sub);
                let state = run_program(&new_prog);

                if state.0 != -1 {
                    println!("Challenge2: {}", state.1);
                    break;
                }
            },
            _ => {},
        }
    }
}

fn run_program(program: &Vec<&str>) -> (isize, isize) {
    let mut state: (isize, isize) = (0, 0);
    let mut done: Vec<isize> = Vec::new();
    while state.0 != -1 && state.0 != program.len() as isize {
        state = run(program[state.0 as usize], &state, &mut done);
    }

    state
}

fn run(instruction: &str, state: &(isize, isize), done: &mut Vec<isize>) -> (isize, isize) {
    let lineno = state.0;
    let acc = state.1;

    if done.contains(&lineno) {
        (-1, acc)
    } else {
        done.push(lineno);
        let mut params = instruction.split(" ");
        let command = params.next().unwrap();
        let arg: isize = params.next().unwrap().parse().unwrap();

        match command {
            "nop" => (lineno + 1, acc),
            "jmp" => (lineno + arg, acc),
            "acc" => (lineno + 1, acc + arg),
            _ => (-1, acc),
        }
    }
}
