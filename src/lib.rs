//! `bfkit` is a Brainfuck development toolkit including an optimizing C compiler and a debugger.

pub mod ir;
pub mod compiler;
pub mod interp;
pub mod optimizer;
pub mod parser;
pub mod repl;