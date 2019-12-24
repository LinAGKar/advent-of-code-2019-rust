use std::io::Read;

fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Div<i32> for Point {
    type Output = Self;

    fn div(self, other: i32) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl std::ops::Mul<i32> for Point {
    type Output = Self;

    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let asteroids: std::collections::HashSet<Point> = input
        .lines()
        .enumerate()
        .flat_map(|(n, x)| x
            .chars()
            .enumerate()
            .filter_map(move |(m, y)| if y == '#' {
                Some(Point { x: m as i32, y: n as i32 })
            } else {
                None
            })
        )
        .collect();

    println!("{}", asteroids.iter().map(|&x| {
        asteroids.iter().filter(|&&y| x != y).filter(|&&y| {
            let diff = y - x;
            let step_count = gcd(diff.x.abs(), diff.y.abs());
            let step = diff / step_count;
            !(1..step_count).any(|z| asteroids.contains(&(x + step * z)))
        }).count()
    }).max().unwrap());
}
