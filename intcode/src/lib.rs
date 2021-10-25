use std::collections::VecDeque;

#[derive(Clone)]
pub struct IntCode {
    memory: Vec<i64>,
    pc: usize,
    relative_base: i64,
    inputs: VecDeque<i64>,
    outputs: VecDeque<i64>,
    last_input: i64,
}

impl IntCode {
    pub fn new(initial_memory: Vec<i64>) -> IntCode {
        IntCode {
            memory: initial_memory,
            pc: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            last_input: 0,
        }
    }

    fn expand_program(&mut self, size: usize) {
        if size > self.memory.len() {
            self.memory.resize(size, 0);
        }
    }

    pub fn get_at_address(&mut self, address: usize) -> i64 {
        self.expand_program(address + 1);
        self.memory[address]
    }

    pub fn set_at_address(&mut self, address: usize, val: i64) {
        self.expand_program(address + 1);
        self.memory[address] = val;
    }

    fn get(&mut self, op: i64, param_offset: usize) -> i64 {
        let param = self.memory[self.pc + param_offset];
        let parameter_mode = (op / 10i64.pow(param_offset as u32 + 1)) % 10;

        match parameter_mode {
            0 => self.get_at_address(param as usize),

            1 => param,

            2 => self.get_at_address((self.relative_base + param) as usize),

            x => panic!("Unknown parameter mode {}", x),
        }
    }

    fn set(&mut self, op: i64, param_offset: usize, val: i64) {
        let param = self.memory[self.pc + param_offset];
        let parameter_mode = (op / 10i64.pow(param_offset as u32 + 1)) % 10;

        match parameter_mode {
            0 => self.set_at_address(param as usize, val),

            1 => panic!("Attempt to set immediate mode parameter"),

            2 => self.set_at_address((self.relative_base + param) as usize, val),

            x => panic!("Unknown parameter mode {}", x),
        }
    }

    pub fn iterate(&mut self) -> bool {
        let op = self.memory[self.pc];
        let opcode = op % 100;

        match opcode {
            1 => {
                let val = self.get(op, 1) + self.get(op, 2);
                self.set(op, 3, val);
                self.pc += 4;
            }

            2 => {
                let val = self.get(op, 1) * self.get(op, 2);
                self.set(op, 3, val);
                self.pc += 4;
            }

            3 => {
                if let Some(x) = self.inputs.pop_front() {
                    self.last_input = x;
                }
                self.set(op, 1, self.last_input);
                self.pc += 2;
            }

            4 => {
                let val = self.get(op, 1);
                self.outputs.push_back(val);
                self.pc += 2;
            }

            5 => if self.get(op, 1) != 0 {
                self.pc = self.get(op, 2) as usize;
            } else {
                self.pc += 3;
            }

            6 => if self.get(op, 1) == 0 {
                self.pc = self.get(op, 2) as usize;
            } else {
                self.pc += 3;
            }

            7 => {
                let val = (self.get(op, 1) < self.get(op, 2)) as i64;
                self.set(op, 3, val);
                self.pc += 4;
            }

            8 => {
                let val = (self.get(op, 1) == self.get(op, 2)) as i64;
                self.set(op, 3, val);
                self.pc += 4;
            }

            9 => {
                self.relative_base += self.get(op, 1);
                self.pc += 2;
            }

            99 => {
                return false;
            }

            x => panic!("Unknown opcode {}", x)
        }

        true
    }

    pub fn put_input(&mut self, input: i64) {
        self.inputs.push_back(input);
    }

    pub fn set_input(&mut self, input: i64) {
        self.inputs.clear();
        self.last_input = input;
    }

    pub fn get_output(&mut self) -> Option<i64> {
        self.outputs.pop_front()
    }

    pub fn get_outputs(&mut self, n: usize) -> Option<Vec<i64>>  {
        if self.outputs.len() >= n {
            Some((0..n).map(|_| self.outputs.pop_front().unwrap()).collect())
        } else {
            None
        }
    }

    pub fn run(&mut self) -> Option<i64> {
        loop {
            if let Some(output) = self.get_output() {
                return Some(output);
            }
            if !self.iterate() {
                return None;
            }
        }
    }
}
