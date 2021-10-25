fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());

    computer.set_at_address(1, 12);
    computer.set_at_address(2, 2);

    while let Some(output) = computer.run() {
        println!("{}", output);
    }

    println!("{}", computer.get_at_address(0));
}
