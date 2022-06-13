use std::io::Read;

fn pow_mod(base: i128, exp: u64, m: i128) -> i128 {
    if exp == 0 {
        1
    } else {
        let t = pow_mod(base, exp / 2, m);
        (if exp % 2 == 0 {
            t * t
        } else {
            (t * t) % m * base
        }) % m
    }
}

fn mod_div(numerator: i128, denominator: i128, m: i128) -> i128 {
    (numerator.rem_euclid(m) * pow_mod(denominator.rem_euclid(m), (m - 2) as u64, m)).rem_euclid(m)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const STACK_SIZE: i128 = 119315717514047;
    const REPETIONS: u64 = 101741582076661;

    let (k, a) = input.lines().fold((1, 0), |(k, a), line| {
        let mut words = line.split_whitespace();
        if words.next().unwrap() == "cut" {
            let count: i128 = words.next().unwrap().parse().unwrap();
            (k, (a - count).rem_euclid(STACK_SIZE))
        } else if words.next().unwrap() == "with" {
            let increment: i128 = words.nth(1).unwrap().parse().unwrap();
            ((k * increment).rem_euclid(STACK_SIZE), (a * increment).rem_euclid(STACK_SIZE))
        } else {
            ((-k).rem_euclid(STACK_SIZE), (-a - 1).rem_euclid(STACK_SIZE))
        }
    });

    let pow = pow_mod(k, REPETIONS, STACK_SIZE);
    let (k, a) = (pow, mod_div(a * (pow - 1), k - 1, STACK_SIZE));
    println!("{}", mod_div(2020 - a, k, STACK_SIZE));
}
