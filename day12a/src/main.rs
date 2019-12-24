use std::io::Read;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl std::ops::SubAssign for Point {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl Point {
    fn energy(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    fn new(x: i32, y: i32, z: i32) -> Point {
        Point {
            x: x,
            y: y,
            z: z,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Moon {
    pos: Point,
    vel: Point,
}

impl Moon {
    fn energy(&self) -> i32 {
        self.pos.energy() * self.vel.energy()
    }

    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            pos: Point::new(x, y, z),
            vel: Point::new(0, 0, 0),
        }
    }

    fn accelerate(&mut self, other: &mut Self) {
        let diff = Point {
            x: match self.pos.x.cmp(&other.pos.x) { Greater => -1, Less => 1, _ => 0 },
            y: match self.pos.y.cmp(&other.pos.y) { Greater => -1, Less => 1, _ => 0 },
            z: match self.pos.z.cmp(&other.pos.z) { Greater => -1, Less => 1, _ => 0 },
        };
        self.vel += diff;
        other.vel -= diff;
    }

    fn translate(&mut self) {
        self.pos += self.vel;
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = regex::Regex::new(r"(?m)^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();
    let mut moons: Vec<Moon> = re.captures_iter(&input).map(|x| {
        Moon::new(x[1].parse().unwrap(), x[2].parse().unwrap(), x[3].parse().unwrap())
    }).collect();

    for _ in 0..1000 {
        for i in 0..(moons.len() - 1) {
            let (moons_a, moons_b) = moons.split_at_mut(i + 1);
            let moon_a = &mut moons_a[i];
            for moon_b in moons_b {
                moon_a.accelerate(moon_b);
            }
        }

        for moon in &mut moons {
            moon.translate();
        }
    }

    println!("{}", moons.iter().map(|x| x.energy()).sum::<i32>());
}
