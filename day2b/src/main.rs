use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let program_orig: Vec<usize> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    for i in 0..100 {
        for j in 0..100 {
            let mut program = program_orig.to_vec();
            let mut pc = 0;
            program[1] = i;
            program[2] = j;
            loop {
                match program[pc] {
                    1 => {
                        let result_pos = program[pc + 3];
                        program[result_pos] = program[program[pc + 1]] + program[program[pc + 2]];
                        pc += 4;
                    }

                    2 => {
                        let result_pos = program[pc + 3];
                        program[result_pos] = program[program[pc + 1]] * program[program[pc + 2]];
                        pc += 4;
                    }

                    99 => {
                    break;
                    }
                
                    _ => panic!("Unknown opcode")
                }
            }
            if program[0] == 19690720 {
                println!("{}", 100 * i + j);
                break;
            }
        }
    }
}
