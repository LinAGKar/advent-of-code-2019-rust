fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut computer = intcode::IntCode::new(input.trim().split(',').map(|x| x.parse().unwrap()).collect());

    let mut block_tiles = std::collections::HashSet::new();

    while computer.iterate() {
        while let Some(output) = computer.get_outputs(3) {
            if output[2] == 2 {
                block_tiles.insert((output[0], output[1]));
            }
        }
    }

    println!("{}", block_tiles.len());
}
