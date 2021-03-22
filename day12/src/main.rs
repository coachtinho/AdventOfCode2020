use std::fs;
use std::f32::consts::PI;

struct Vector2D {
    x: isize,
    y: isize,
}

impl Vector2D {
    fn rotate(&mut self, radian: f32) {
        let newx = self.x * radian.cos() as isize - self.y * radian.sin() as isize;
        let newy = self.x * radian.sin() as isize + self.y * radian.cos() as isize;
        self.x = newx;
        self.y = newy;
    }

    fn add(&mut self, other: Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }

    fn mul(&mut self, value: isize) {
        self.x *= value;
        self.y *= value;
    }
}

impl Clone for Vector2D {
    fn clone(&self) -> Self {
        Self { x: self.x, y: self.y }
    }
}

struct Ferry {
    pos: Vector2D,
    vel: Vector2D,
}

impl Ferry {
    fn new() -> Self {
        Self {
            pos: Vector2D { x: 0, y: 0 },
            vel: Vector2D { x: 1, y: 0 },
        }
    }

    fn forward(&mut self, value: isize) {
        self.pos.x += self.vel.x * value;
        self.pos.y += self.vel.y * value;
    }

    fn strafe(&mut self, vel: Vector2D) {
        self.pos.add(vel);
    }

    fn turn(&mut self, degree: isize) {
        let radian = degree_to_radian(degree);
        self.vel.rotate(radian);
    }

    fn get_man_distance(&self) -> isize {   
        self.pos.x.abs() + self.pos.y.abs()
    }
}

fn degree_to_radian(degree: isize) -> f32 {
    degree as f32 * PI / 180_f32
}


fn main() {
    // Read input from file
    let contents = fs::read_to_string("input.txt").expect("Failed reading input file");

    // Challenge1
    let mut ferry = Ferry::new();
    contents.lines().for_each(|line| {
        let mut s = String::from(line);
        match s.remove(0) {
            'N' => {
                let n:isize = s.parse().unwrap();
                ferry.strafe(Vector2D { x: 0, y: -1 * n });
            },
            'S' => {
                let n:isize = s.parse().unwrap();
                ferry.strafe(Vector2D { x: 0, y: n });
            },
            'E' => {
                let n:isize = s.parse().unwrap();
                ferry.strafe(Vector2D { x: n, y: 0 });
            },
            'W' => {
                let n:isize = s.parse().unwrap();
                ferry.strafe(Vector2D { x: -1 * n, y: 0 });
            },
            'L' => {
                let n:isize = s.parse().unwrap();
                ferry.turn(-1 * n);
            },
            'R' => {
                let n:isize = s.parse().unwrap();
                ferry.turn(n);
            },
            'F' => {
                let n:isize = s.parse().unwrap();
                ferry.forward(n);
            },
            _ => {}
        };
    });
    println!("Challenge1: {}", ferry.get_man_distance());

    // Challenge2
    let mut ferry = Ferry::new();
    let mut waypoint = Vector2D { x: 10, y: -1 };
    contents.lines().for_each(|line| {
        let mut s = String::from(line);
        match s.remove(0) {
            'N' => {
                let n:isize = s.parse().unwrap();
                waypoint.add(Vector2D { x: 0, y: -1 * n });
            },
            'S' => {
                let n:isize = s.parse().unwrap();
                waypoint.add(Vector2D { x: 0, y: n });
            },
            'E' => {
                let n:isize = s.parse().unwrap();
                waypoint.add(Vector2D { x: n, y: 0 });
            },
            'W' => {
                let n:isize = s.parse().unwrap();
                waypoint.add(Vector2D { x: -1 * n, y: 0 });
            },
            'L' => {
                let n:isize = s.parse().unwrap();
                waypoint.rotate(degree_to_radian(-1 * n));
            },
            'R' => {
                let n:isize = s.parse().unwrap();
                waypoint.rotate(degree_to_radian(n));
            },
            'F' => {
                let n:isize = s.parse().unwrap();
                let mut vel = waypoint.clone();
                vel.mul(n);
                ferry.strafe(vel);
            },
            _ => {}
        };
    });
    println!("Challenge2: {}", ferry.get_man_distance());
}
