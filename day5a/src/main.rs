fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());
    computer.put_input(1);
    let mut prev_output = 0;

    while computer.iterate() {
        while let Some(output) = computer.get_output() {
            assert_eq!(prev_output, 0);
            prev_output = output;
        }
    }

    println!("{}", prev_output);
}
