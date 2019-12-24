use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut program: Vec<i32> = input.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut pc = 0;
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
                set(
                    program[pc + 1],
                    5,
                    (op / 100) % 10,
                    &mut program,
                );
                pc += 2;
            }

            4 => {
                println!("{}", get(
                    program[pc + 1],
                    (op / 100) % 10,
                    &program,
                ));
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
