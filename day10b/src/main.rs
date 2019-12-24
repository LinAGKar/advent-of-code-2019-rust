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

fn reachable_asteroids<'a>(
        pos: Point,
        asteroids: &'a std::collections::HashSet<Point>,
) -> Box<dyn Iterator<Item=&Point> + 'a> {
    Box::new(asteroids.iter().filter(move |&&x| pos != x).filter(move |&&x| {
        let diff = x - pos;
        let step_count = gcd(diff.x.abs(), diff.y.abs());
        let step = diff / step_count;
        !(1..step_count).any(|y| asteroids.contains(&(pos + step * y)))
    }))
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut asteroids: std::collections::HashSet<Point> = input
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

    let station_pos = asteroids.iter().map(|&x| {
        (x, reachable_asteroids(x, &asteroids).count())
    }).max_by_key(|&(_pos, count)| count).unwrap().0;

    const N: usize = 200;
    let mut destroyed = 0;
    loop {
        let reachable: Vec<Point> = reachable_asteroids(station_pos, &asteroids).map(|&x| x).collect();
        if destroyed + reachable.len() < N {
            destroyed += reachable.len();
            for i in reachable {
                asteroids.remove(&i);
            }
        } else {
            let mut reachable_with_angles: Vec<(Point, f64)> = reachable.iter().map(|&x| {
                let diff = x - station_pos;
                let angle = ((diff.y as f64).atan2(diff.x as f64) +
                             std::f64::consts::PI / 2.0 +
                             2.0 * std::f64::consts::PI
                             ) % (2.0 * std::f64::consts::PI);
                (x, angle)
            }).collect();
            reachable_with_angles.sort_by(|&(_, angle_a), (_, angle_b)| angle_a.partial_cmp(angle_b).unwrap());
            let nth = reachable_with_angles[N - destroyed - 1].0;
            println!("{}", nth.x * 100 + nth.y);
            break;
        }
    }
}
