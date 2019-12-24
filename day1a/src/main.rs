use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    println!("{}", input.lines().map(|x| x.parse::<i32>().unwrap() / 3 - 2).fold(0, |acc, x| acc + x));
}
