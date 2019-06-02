use crate::parser::BrainfuckInstruction;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Read, Write};

pub enum StopReason {
    Breakpoint(usize),
    Done,
}

pub struct Interpreter {
    code: Vec<BrainfuckInstruction>,
    tape: Vec<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
    breakpoints: HashSet<usize>,
    jump_table: HashMap<usize, usize>,
}

impl Interpreter {
    pub fn new(code: Vec<BrainfuckInstruction>) -> Self {
        let mut result = Self {
            code,
            tape: vec![0u8; 3000],
            data_pointer: 0,
            instruction_pointer: 0,
            breakpoints: HashSet::new(),
            jump_table: HashMap::new(),
        };
        result.compute_jump_table();
        result
    }

    fn compute_jump_table(&mut self) {
        let mut stack = Vec::new();
        let mut index = 0;
        while index < self.code.len() {
            match self.code[index] {
                BrainfuckInstruction::Open => stack.push(index),
                BrainfuckInstruction::Close => {
                    let other = stack.pop().expect("Got a ] with no matching [");
                    self.jump_table.insert(index, other + 1);
                    self.jump_table.insert(other, index + 1);
                }
                _ => {}
            }
            index += 1;
        }

        if !stack.is_empty() {
            panic!("Got a [ with no matching ]");
        }
    }

    pub fn set_breakpoint(&mut self, address: usize) -> Result<bool, String> {
        if address >= self.code.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            Ok(self.breakpoints.insert(address))
        }
    }

    pub fn delete_breakpoint(&mut self, address: usize) -> bool {
        self.breakpoints.remove(&address)
    }

    pub fn run(&mut self) -> StopReason {
        while self.instruction_pointer < self.code.len() {
            if self.breakpoints.contains(&self.instruction_pointer) {
                return StopReason::Breakpoint(self.instruction_pointer);
            } else {
                self.step();
            }
        }
        StopReason::Done
    }

    pub fn step(&mut self) {
        let mut next_instruction_pointer = self.instruction_pointer + 1;

        match self.code[self.instruction_pointer] {
            BrainfuckInstruction::Add(count) => {
                self.tape[self.data_pointer] = self.tape[self.data_pointer].wrapping_add(count)
            }
            BrainfuckInstruction::Sub(count) => {
                self.tape[self.data_pointer] = self.tape[self.data_pointer].wrapping_sub(count)
            }
            BrainfuckInstruction::Right(count) => self.data_pointer += count,
            BrainfuckInstruction::Left(count) => self.data_pointer -= count,
            BrainfuckInstruction::Read => {
                self.tape[self.data_pointer] = stdin().bytes().take(1).last().unwrap().unwrap()
            }
            BrainfuckInstruction::Write => {
                print!("{}", self.tape[self.data_pointer] as char);
                stdout().flush().unwrap();
            }
            BrainfuckInstruction::Open => {
                if self.tape[self.data_pointer] == 0 {
                    next_instruction_pointer = self.jump_table[&self.instruction_pointer];
                }
            }
            BrainfuckInstruction::Close => {
                if self.tape[self.data_pointer] != 0 {
                    next_instruction_pointer = self.jump_table[&self.instruction_pointer];
                }
            }
            BrainfuckInstruction::Set(value) => {
                self.tape[self.data_pointer] = value;
            }
            BrainfuckInstruction::ScanRight => {
                while self.tape[self.data_pointer] != 0 && self.data_pointer < self.tape.len() {
                    self.data_pointer += 1;
                }
            }
            BrainfuckInstruction::ScanLeft => {
                while self.tape[self.data_pointer] != 0 && self.data_pointer > 0 {
                    self.data_pointer -= 1;
                }
            }
        }

        self.instruction_pointer = next_instruction_pointer;
    }

    pub fn get(&self, address: usize) -> Result<u8, String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            Ok(self.tape[address])
        }
    }

    pub fn set(&mut self, address: usize, value: u8) -> Result<u8, String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            let old = self.tape[address];
            self.tape[address] = value;
            Ok(old)
        }
    }

    pub fn jump(&mut self, address: usize) -> Result<(), String> {
        if address >= self.code.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            self.instruction_pointer = address;
            Ok(())
        }
    }

    pub fn select(&mut self, address: usize) -> Result<(), String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            self.data_pointer = address;
            Ok(())
        }
    }
}
