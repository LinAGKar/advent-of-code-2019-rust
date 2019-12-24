use std::io::Read;
use std::cmp::Ordering::Greater;
use std::cmp::Ordering::Less;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = regex::Regex::new(r"(?m)^<x=(-?\d+), y=(-?\d+), z=(-?\d+)>$").unwrap();

    let mut axes = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for i in re.captures_iter(&input) {
        for j in 0..3 {
            axes[j].push((i[j + 1].parse::<i32>().unwrap(), 0));
        }
    }

    let initial: Vec<_> = axes.iter().map(|x| x.to_vec()).collect();

    let repeats: Vec<_> = axes.iter_mut().enumerate().map(|(n, x)| {
        let mut count = 0;
        loop {
            for i in 0..(x.len() - 1) {
                let (moons_a, moons_b) = x.split_at_mut(i + 1);
                let moon_a = &mut moons_a[i];
                for moon_b in moons_b {
                    let diff = match moon_a.0.cmp(&moon_b.0) { Greater => -1, Less => 1, _ => 0 };
                    moon_a.1 += diff;
                    moon_b.1 -= diff;
                }
            }

            for moon in x.iter_mut() {
                moon.0 += moon.1;
            }

            count += 1;

            if *x == initial[n] {
                break;
            }
        }
        count
    }).collect();

    println!("{}", lcm(lcm(repeats[0], repeats[1]), repeats[2]));
}
