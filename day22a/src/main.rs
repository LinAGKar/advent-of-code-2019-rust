use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    const STACK_SIZE: i32 = 10007;

    println!("{}", input.lines().fold(2019, |acc, line| {
        let mut words = line.split_whitespace();
        if words.next().unwrap() == "cut" {
            let count: i32 = words.next().unwrap().parse().unwrap();
            (acc - count).rem_euclid(STACK_SIZE)
        } else if words.next().unwrap() == "with" {
            let increment: i32 = words.nth(1).unwrap().parse().unwrap();
            (acc * increment).rem_euclid(STACK_SIZE)
        } else {
            STACK_SIZE - acc - 1
        }
    }));
}
