fn construct_phases(selection: &[u8; 4]) -> [u8; 5] {
    let mut phases = vec![0, 1, 2, 3, 4];
    let mut result = [0; 5];
    for (n, i) in selection.iter().map(|&x| phases.remove(x as usize)).enumerate() {
        result[n] = i;
    }
    result[4] = phases[0];
    result
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let initial_program: Vec<i64> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut greatest_output = 0;

    for i in 0..5 * 4 * 3 * 2 {
        let mut selection = [0; 4];
        let mut select = i;
        for j in 0..4 {
            selection[j] = (select % (5 - j)) as u8;
            select /= 5 - j;
        }

        let mut signal = 0;

        for &phase in &construct_phases(&selection) {
            let mut computer = intcode::IntCode::new(initial_program.to_vec());
            computer.put_input(phase as i64);
            computer.put_input(signal);

            while let Some(output) = computer.run() {
                signal = output;
            }
        }

        greatest_output = std::cmp::max(greatest_output, signal);
    }
    println!("{}", greatest_output);
}
