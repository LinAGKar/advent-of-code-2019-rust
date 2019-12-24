use std::cmp::max;
use std::io;

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
    io::stdin().read_line(&mut input).unwrap();
    let initial_program: Vec<i32> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
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
            let mut program = initial_program.to_vec();
            let mut pc = 0;
            let mut has_sent_phase = false;
            loop {
                let op = program[pc];
                let opcode = op % 100;

                fn get(param: i32, parameter_mode: i32, mem: &Vec<i32>) -> i32 {
                    match parameter_mode {
                        0 => mem[param as usize],

                        1 => param,

                        x => panic!("Unknown parameter mode {}", x),
                    }
                };

                fn set(param: i32, val: i32, parameter_mode: i32, mem: &mut Vec<i32>) {
                    match parameter_mode {
                        0 => mem[param as usize] = val,

                        1 => {},

                        x => panic!("Unknown parameter mode {}", x),
                    }
                };

                match opcode {
                    1 => {
                        set(
                            program[pc + 3],
                            get(program[pc + 1], (op / 100) % 10, &program) + get(program[pc + 2], (op / 1000) % 10, &program),
                            (op / 10000) % 10,
                            &mut program,
                        );
                        pc += 4;
                    }

                    2 => {
                        set(
                            program[pc + 3],
                            get(program[pc + 1], (op / 100) % 10, &program) * get(program[pc + 2], (op / 1000) % 10, &program),
                            (op / 10000) % 10,
                            &mut program,
                        );
                        pc += 4;
                    }

                    3 => {
                        let input = if has_sent_phase {
                            signal
                        } else {
                            has_sent_phase = true;
                            phase as i32
                        };
                        set(
                            program[pc + 1],
                            input,
                            (op / 100) % 10,
                            &mut program,
                        );
                        pc += 2;
                    }

                    4 => {
                        signal = get(
                            program[pc + 1],
                            (op / 100) % 10,
                            &program,
                        );
                        pc += 2;
                    }

                    5 => if get(program[pc + 1], (op / 100) % 10, &program) != 0 {
                        pc = get(program[pc + 2], (op / 1000) % 10, &program) as usize;
                    } else {
                        pc += 3;
                    }

                    6 => if get(program[pc + 1], (op / 100) % 10, &program) == 0 {
                        pc = get(program[pc + 2], (op / 1000) % 10, &program) as usize;
                    } else {
                        pc += 3;
                    }

                    7 => {
                        set(
                            program[pc + 3],
                            (get(program[pc + 1], (op / 100) % 10, &program) < get(program[pc + 2], (op / 1000) % 10, &program)) as i32,
                            (op / 10000) % 10,
                            &mut program,
                        );
                        pc += 4;
                    }

                    8 => {
                        set(
                            program[pc + 3],
                            (get(program[pc + 1], (op / 100) % 10, &program) == get(program[pc + 2], (op / 1000) % 10, &program)) as i32,
                            (op / 10000) % 10,
                            &mut program,
                        );
                        pc += 4;
                    }

                    99 => {
                        break;
                    }

                    x => panic!("Unknown opcode {}", x)
                }
            }
        }

        greatest_output = max(greatest_output, signal);
    }
    println!("{}", greatest_output);
}
