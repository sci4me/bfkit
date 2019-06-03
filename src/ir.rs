//! The Intermediate Representation used by bfkit to represent Brainfuck code.

/// Represents any of the eight standard Brainfuck instructions:
///
/// * `+`
/// * `-`
/// * `>`
/// * `<`
/// * `,`
/// * `.`
/// * `[`
/// * `]`
///
/// as well as any instructions generated by optimizations.
#[derive(Debug, Clone, PartialEq)]
pub enum BrainfuckInstruction {
    /// Add represents some number of Brainfuck `+` instructions.
    Add(u8),
    /// Sub represents some number of Brainfuck `-` instructions.
    Sub(u8),
    /// Right represents some number of Brainfuck '>' instructions.
    Right(usize),
    /// Left represents some number of Brainfuck '<' instructions.
    Left(usize),
    /// Read represents a Brainfuck `,` instruction.
    Read,
    /// Write represents a Brainfuck `.` instruction.
    Write,
    /// Open represents a Brainfuck `[` instruction.
    Open,
    /// Close represents a Brainfuck `]` instruction.
    Close,
    /// Set is an instruction that assigns a cell in the tape to some value.
    Set(u8),
    /// ScanLeft represents the following sequence of Brainfuck instructions: `[<]`
    ScanLeft,
    /// ScanRight represents the following sequence of Brainfuck instructions: `[>]`
    ScanRight,
}

/// Converts a sequence of BrainfuckInstructions to a string.
///
/// # Arguments
///
/// * `ir` - The sequence of BrainfuckInstructions to be stringified.
pub fn ir_to_string(ir: Vec<BrainfuckInstruction>) -> String {
    let mut result = String::new();
    let mut level = 0;

    let indent = |result: &mut String, level: i32| {
        for _ in 0..level {
            result.push_str("    ");
        }
    };

    for insn in ir {
        match insn {
            BrainfuckInstruction::Add(count) => {
                indent(&mut result, level);
                result.push_str(&format!("add {}\n", count))
            },
            BrainfuckInstruction::Sub(count) => {
                indent(&mut result, level);
                result.push_str(&format!("sub {}\n", count))
            },
            BrainfuckInstruction::Right(count) => {
                indent(&mut result, level);
                result.push_str(&format!("right {}\n", count))
            },
            BrainfuckInstruction::Left(count) => {
                indent(&mut result, level);
                result.push_str(&format!("left {}\n", count))
            },
            BrainfuckInstruction::Read => {
                indent(&mut result, level);
                result.push_str("read\n")
            },
            BrainfuckInstruction::Write => {
                indent(&mut result, level);
                result.push_str("write\n")
            },
            BrainfuckInstruction::Open => {
                indent(&mut result, level);
                level += 1;
                result.push_str("open\n")
            },
            BrainfuckInstruction::Close => {
                level -= 1;
                indent(&mut result, level);
                result.push_str("close\n")
            },
            BrainfuckInstruction::Set(value) => {
                indent(&mut result, level);
                result.push_str(&format!("set {}\n", value))
            },
            BrainfuckInstruction::ScanRight => {
                indent(&mut result, level);
                result.push_str("scan_right\n")
            },
            BrainfuckInstruction::ScanLeft => {
                indent(&mut result, level);
                result.push_str("scan_left\n")
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;
    use crate::optimizer;

    #[test]
    fn ir_to_string_works() {
        let code = optimizer::optimize(parser::parse_str(String::from("++-[-],.[>++<-][<][>]")), 10);

        let result = ir_to_string(code);

        let expected = vec![
            "add 2",
            "sub 1",
            "set 0",
            "read",
            "write",
            "open",
            "    right 1",
            "    add 2",
            "    left 1",
            "    sub 1",
            "close",
            "scan_left",
            "scan_right",
        ]
            .iter()
            .fold(String::new(), |a, b| format!("{}\n{}", a, b));

        assert_eq!(result.trim(), expected.trim());
    }
}