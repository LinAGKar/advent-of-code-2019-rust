fn construct_phases(selection: &[u8; 4]) -> [u8; 5] {
    let mut phases = vec![5, 6, 7, 8, 9];
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

        let mut amplifiers: Vec<intcode::IntCode> = construct_phases(&selection).iter().map(|&x| {
            let mut amp = intcode::IntCode::new(initial_program.to_vec());
            amp.put_input(x as i64);
            amp
        }).collect();

        let mut current_amplifier = 0;

        loop {
            let amp = amplifiers.get_mut(current_amplifier).unwrap();
            current_amplifier = (current_amplifier + 1) % 5;
            amp.put_input(signal);

            if let Some(output) = amp.run() {
                signal = output;
            } else {
                break;
            }
        }

        greatest_output = std::cmp::max(greatest_output, signal);
    }
    println!("{}", greatest_output);
}
