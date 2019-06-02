use crate::parser::BrainfuckInstruction;
use std::collections::HashSet;

pub enum StopReason {
    Breakpoint(usize),
    Done
}

pub struct Interpreter {
    code: Vec<BrainfuckInstruction>,
    tape: Vec<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
    breakpoints: HashSet<usize>
}

impl Interpreter {
    pub fn new(code: Vec<BrainfuckInstruction>) -> Self {
        Self {
            code,
            tape: vec![0u8; 3000],
            data_pointer: 0,
            instruction_pointer: 0,
            breakpoints: HashSet::new()
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
        unimplemented!()
    }

    pub fn step(&mut self) {
        unimplemented!()
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
}