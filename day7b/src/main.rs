use std::cmp::max;
use std::io;

fn construct_phases(selection: &[u8; 4]) -> [u8; 5] {
    let mut phases = vec![5, 6, 7, 8, 9];
    let mut result = [0; 5];
    for (n, i) in selection.iter().map(|&x| phases.remove(x as usize)).enumerate() {
        result[n] = i;
    }
    result[4] = phases[0];
    result
}

struct Amplifier {
    phase: i32,
    program: Vec<i32>,
    pc: usize,
    has_sent_phase: bool,
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

        let mut amplifiers: Vec<Amplifier> = construct_phases(&selection).iter().map(|&x| {
            Amplifier {
                phase: x as i32,
                program: initial_program.to_vec(),
                pc: 0,
                has_sent_phase: false,
            }
        }).collect();

        let mut current_amplifier = 0;

        'amploop: loop {
            let amp = amplifiers.get_mut(current_amplifier).unwrap();
            current_amplifier = (current_amplifier + 1) % 5;

            loop {
                let op = amp.program[amp.pc];
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
                            amp.program[amp.pc + 3],
                            get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) + get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program),
                            (op / 10000) % 10,
                            &mut amp.program,
                        );
                        amp.pc += 4;
                    }

                    2 => {
                        set(
                            amp.program[amp.pc + 3],
                            get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) * get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program),
                            (op / 10000) % 10,
                            &mut amp.program,
                        );
                        amp.pc += 4;
                    }

                    3 => {
                        let input = if amp.has_sent_phase {
                            signal
                        } else {
                            amp.has_sent_phase = true;
                            amp.phase
                        };
                        set(
                            amp.program[amp.pc + 1],
                            input,
                            (op / 100) % 10,
                            &mut amp.program,
                        );
                        amp.pc += 2;
                    }

                    4 => {
                        signal = get(
                            amp.program[amp.pc + 1],
                            (op / 100) % 10,
                            &amp.program,
                        );
                        amp.pc += 2;
                        break;
                    }

                    5 => if get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) != 0 {
                        amp.pc = get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program) as usize;
                    } else {
                        amp.pc += 3;
                    }

                    6 => if get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) == 0 {
                        amp.pc = get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program) as usize;
                    } else {
                        amp.pc += 3;
                    }

                    7 => {
                        set(
                            amp.program[amp.pc + 3],
                            (get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) < get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program)) as i32,
                            (op / 10000) % 10,
                            &mut amp.program,
                        );
                        amp.pc += 4;
                    }

                    8 => {
                        set(
                            amp.program[amp.pc + 3],
                            (get(amp.program[amp.pc + 1], (op / 100) % 10, &amp.program) == get(amp.program[amp.pc + 2], (op / 1000) % 10, &amp.program)) as i32,
                            (op / 10000) % 10,
                            &mut amp.program,
                        );
                        amp.pc += 4;
                    }

                    99 => {
                        break 'amploop;
                    }

                    x => panic!("Unknown opcode {}", x)
                }
            }
        }

        greatest_output = max(greatest_output, signal);
    }
    println!("{}", greatest_output);
}
