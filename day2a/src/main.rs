use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut program: Vec<usize> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut pc = 0;
    program[1] = 12;
    program[2] = 2;
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
    println!("{}", program[0]);
}
