use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.lines().map(|x| {
        let mut fuel = 0;
        let mut mass = x.parse::<i32>().unwrap();
        while mass > 0 {
            mass = mass / 3 - 2;
            if mass <= 0 {
                break;
            }
            fuel += mass;
        }
        fuel
    }).fold(0, |acc, x| acc + x));
}
