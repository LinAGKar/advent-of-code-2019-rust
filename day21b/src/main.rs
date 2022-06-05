fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());

    let script = "\
        NOT A J\n\
        NOT B T\n\
        OR T J\n\
        NOT C T\n\
        OR T J\n\
        AND D J\n\
        NOT E T\n\
        NOT T T\n\
        OR H T\n\
        AND T J\n\
        RUN\n\
    ";

    for i in script.chars() {
        computer.put_input(i as i64);
    }

    while let Some(output) = computer.run() {
        if output >= 128 {
            println!("{}", output);
            break;
        }
    }
}
