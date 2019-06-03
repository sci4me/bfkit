//! A Brainfuck interpreter that supports breakpoints.

use crate::ir::BrainfuckInstruction;
use std::collections::{HashMap, HashSet};
use std::io::{stdin, stdout, Read, Write};

/// StopReason represents a reason the Interpreter might stop running.
#[derive(Debug, PartialEq)]
pub enum StopReason {
    /// Breakpoint means that the Interpreter tried to execute an instruction that has a breakpoint set on it.
    Breakpoint(usize),
    /// Done means that the Interpreter finished executing the Brainfuck program.
    Done,
}

/// A Brainfuck interpreter that supports breakpoints.
pub struct Interpreter {
    code: Vec<BrainfuckInstruction>,
    tape: Vec<u8>,
    data_pointer: usize,
    instruction_pointer: usize,
    breakpoints: HashSet<usize>,
    jump_table: HashMap<usize, usize>,
}

impl Interpreter {
    /// Creates a new Interpreter form some input program.
    ///
    /// # Arguments
    ///
    /// * `code` - A sequence of BrainfuckInstructions to be interpreted.
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

    /// Sets a breakpoint at the specified code address.
    ///
    /// # Arguments
    ///
    /// * `address` - The code address to set a breakpoint at.
    pub fn set_breakpoint(&mut self, address: usize) -> Result<bool, String> {
        if address >= self.code.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            Ok(self.breakpoints.insert(address))
        }
    }

    /// Removes a breakpoint at the specified code address.
    ///
    /// # Arguments
    ///
    /// * `address` - The code address to delete a breakpoint from.
    pub fn delete_breakpoint(&mut self, address: usize) -> bool {
        self.breakpoints.remove(&address)
    }

    /// Runs the Interpreter until either a breakpoint is hit or until the program has run to completion.
    pub fn run(&mut self) -> StopReason {
        while self.instruction_pointer < self.code.len() {
            self.step();

            if self.breakpoints.contains(&self.instruction_pointer) {
                return StopReason::Breakpoint(self.instruction_pointer);
            }
        }
        StopReason::Done
    }

    /// Executes a single BrainfuckInstruction.
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

    /// Reads a value from the tape at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The tape address to read.
    pub fn get(&self, address: usize) -> Result<u8, String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            Ok(self.tape[address])
        }
    }

    /// Writes a value to the tape at the specified address.
    ///
    /// # Arguments
    ///
    /// * `address` - The tape address to write to.
    /// * `value` - The value to write.
    pub fn set(&mut self, address: usize, value: u8) -> Result<u8, String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            let old = self.tape[address];
            self.tape[address] = value;
            Ok(old)
        }
    }

    /// Sets the instruction pointer to the specified code address.
    ///
    /// # Arguments
    ///
    /// * `address` - The code address to jump to.
    pub fn jump(&mut self, address: usize) -> Result<(), String> {
        if address >= self.code.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            self.instruction_pointer = address;
            Ok(())
        }
    }

    /// Sets the data pointer to the specified tape address.
    ///
    /// # Arguments
    ///
    /// * `address` - The tape address to set the data pointer to.
    pub fn select(&mut self, address: usize) -> Result<(), String> {
        if address >= self.tape.len() {
            Err(format!("Address out of bounds: {}", address))
        } else {
            self.data_pointer = address;
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    #[test]
    fn set_breakpoint_works() {
        let code = parser::parse_str(String::from("++[>++<-]"));
        let mut subject = Interpreter::new(code);

        subject.set_breakpoint(3);

        assert!(subject.breakpoints.contains(&3))
    }

    #[test]
    fn delete_breakpoint_works() {
        let code = parser::parse_str(String::from("++[>++<-]"));
        let mut subject = Interpreter::new(code);
        subject.breakpoints.insert(3);

        subject.delete_breakpoint(3);

        assert!(!subject.breakpoints.contains(&3));
    }

    #[test]
    fn run_works() {
        let code = parser::parse_str(String::from("++[>++<-]"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.run(), StopReason::Done);

        assert_eq!(subject.tape[0], 0);
        assert_eq!(subject.tape[1], 4);
        assert_eq!(subject.instruction_pointer, 9);
        assert_eq!(subject.data_pointer, 0);
    }

    #[test]
    fn run_stops_for_breakpoints() {
        let code = parser::parse_str(String::from("++[>++<-]"));
        let mut subject = Interpreter::new(code);

        subject.set_breakpoint(1);

        assert_eq!(subject.run(), StopReason::Breakpoint(1));
        assert_eq!(subject.run(), StopReason::Done);

        assert_eq!(subject.tape[0], 0);
        assert_eq!(subject.tape[1], 4);
        assert_eq!(subject.instruction_pointer, 9);
        assert_eq!(subject.data_pointer, 0);
    }

    #[test]
    fn step_works() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.tape[0], 0);
        subject.step();
        assert_eq!(subject.tape[0], 1);
        subject.step();
        assert_eq!(subject.tape[0], 2);
        subject.step();
        assert_eq!(subject.tape[0], 3);
    }

    #[test]
    fn get_works() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        subject.tape[42] = 42;

        assert_eq!(subject.get(42), Ok(42));
    }

    #[test]
    fn get_returns_error_when_address_out_of_bounds() {
        let code = parser::parse_str(String::from("+++"));
        let subject = Interpreter::new(code);

        assert_eq!(subject.get(100000000), Err(String::from("Address out of bounds: 100000000")));
    }

    #[test]
    fn set_works() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        subject.tape[42] = 12;

        assert_eq!(subject.set(42, 42), Ok(12));
    }

    #[test]
    fn set_returns_error_when_address_out_of_bounds() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.set(100000000, 42), Err(String::from("Address out of bounds: 100000000")));
    }

    #[test]
    fn jump_works() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.jump(2), Ok(()));
        assert_eq!(subject.instruction_pointer, 2);
    }

    #[test]
    fn jump_returns_error_when_address_out_of_bounds() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.jump(100000000), Err(String::from("Address out of bounds: 100000000")));
        assert_eq!(subject.instruction_pointer, 0);
    }

    #[test]
    fn select_works() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.select(2), Ok(()));
        assert_eq!(subject.data_pointer, 2);
    }

    #[test]
    fn select_returns_error_when_address_out_of_bounds() {
        let code = parser::parse_str(String::from("+++"));
        let mut subject = Interpreter::new(code);

        assert_eq!(subject.select(100000000), Err(String::from("Address out of bounds: 100000000")));
        assert_eq!(subject.data_pointer, 0);
    }
}