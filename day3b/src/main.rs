use std::collections::HashMap;
use std::io;
use std::ops::AddAssign;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

fn main() {
    let mut wire_a = HashMap::new();
    let mut closest_intersection = std::i32::MAX;
    for i in 0..2 {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut pos = Point { x: 0, y: 0 };
        let mut steps = 0;
        for j in input.trim().split(',') {
            let length: usize = j.chars().skip(1).collect::<String>().parse().unwrap();
            let direction = match j.chars().nth(0).unwrap() {
                'U' => Point { x: 0, y: 1 },

                'D' => Point { x: 0, y: -1 },

                'L' => Point { x: -1, y: 0 },

                'R' => Point { x: 1, y: 0 },

                _ => panic!("Unknown token"),
            };
            for _ in 0..length {
                pos += direction;
                steps += 1;
                if i == 0 {
                    wire_a.insert(pos, steps);
                } else {
                    if let Some(steps_a) = wire_a.get(&pos) {
                        let distance = steps_a + steps;
                        if distance < closest_intersection {
                            closest_intersection = distance;
                        }
                    }
                }
            }
        }
    }
    println!("{}", closest_intersection);
}
