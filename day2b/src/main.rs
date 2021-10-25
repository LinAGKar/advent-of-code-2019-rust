fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let program_orig: Vec<i64> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    for i in 0..100 {
        for j in 0..100 {
            let mut computer = intcode::IntCode::new(program_orig.to_vec());

            computer.set_at_address(1, i);
            computer.set_at_address(2, j);

            while let Some(output) = computer.run() {
                println!("{}", output);
            }
            if computer.get_at_address(0) == 19690720 {
                println!("{}", 100 * i + j);
                break;
            }
        }
    }
}
