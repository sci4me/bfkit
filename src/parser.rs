//! A parser from Brainfuck source code to sequences of BrainfuckInstructions.

use crate::ir::BrainfuckInstruction;

/// Parses a sequence of BrainfuckInstructions from a string.
/// Ignores all non-Brainfuck characters.
///
/// # Arguments
///
/// * `code` - The Brainfuck source code to parse.
pub fn parse_str(code: String) -> Vec<BrainfuckInstruction> {
    parse(&code.chars().collect::<Vec<char>>())
}

/// Parses a sequence of BrainfuckInstructions from a slice of characters.
/// Ignores all non-Brainfuck characters.
///
/// # Arguments
///
/// * `code` - The Brainfuck source code to parse.
pub fn parse(code: &[char]) -> Vec<BrainfuckInstruction> {
    let mut result = Vec::new();

    let mut index = 0;
    while index < code.len() {
        let c = code[index];
        index += 1;
        match c {
            '+' => result.push(BrainfuckInstruction::Add(1)),
            '-' => result.push(BrainfuckInstruction::Sub(1)),
            '>' => result.push(BrainfuckInstruction::Right(1)),
            '<' => result.push(BrainfuckInstruction::Left(1)),
            ',' => result.push(BrainfuckInstruction::Read),
            '.' => result.push(BrainfuckInstruction::Write),
            '[' => result.push(BrainfuckInstruction::Open),
            ']' => result.push(BrainfuckInstruction::Close),
            _ => {}
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_str_parses_brainfuck_instructions() {
        let code = "++--,.[]<<>> [-]";

        let result = parse_str(code.to_string());

        assert_eq!(
            result,
            vec![
                BrainfuckInstruction::Add(1),
                BrainfuckInstruction::Add(1),
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Read,
                BrainfuckInstruction::Write,
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Close,
                BrainfuckInstruction::Left(1),
                BrainfuckInstruction::Left(1),
                BrainfuckInstruction::Right(1),
                BrainfuckInstruction::Right(1),
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Close
            ]
        );
    }

    #[test]
    fn parse_parses_brainfuck_instructions() {
        let code = "++--,.[]<<>> [-]";

        let result = parse(&code.chars().collect::<Vec<char>>());

        assert_eq!(
            result,
            vec![
                BrainfuckInstruction::Add(1),
                BrainfuckInstruction::Add(1),
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Read,
                BrainfuckInstruction::Write,
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Close,
                BrainfuckInstruction::Left(1),
                BrainfuckInstruction::Left(1),
                BrainfuckInstruction::Right(1),
                BrainfuckInstruction::Right(1),
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Close
            ]
        );
    }
}
