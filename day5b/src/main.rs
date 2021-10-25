fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());
    computer.put_input(5);

    while let Some(output) = computer.run() {
        println!("{}", output);
    }
}
