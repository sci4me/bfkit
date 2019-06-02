use crate::parser::BrainfuckInstruction;

/// Compiles a sequence of BrainfuckInstructions to C source code.
pub fn compile(ir: Vec<BrainfuckInstruction>) -> String {
    let mut result = String::new();

    fn indent(s: &mut String, n: u32) {
        for _ in 0..n {
            s.push_str("    ");
        }
    }

    let mut level = 1;
    let mut index = 0;
    while index < ir.len() {
        match &ir[index] {
            BrainfuckInstruction::Add(count) => {
                indent(&mut result, level);
                result.push_str(&format!("ADJUST(0, {})", count));
            }
            BrainfuckInstruction::Sub(count) => {
                indent(&mut result, level);
                result.push_str(&format!("ADJUST(0, -{})", count));
            }
            BrainfuckInstruction::Right(count) => {
                indent(&mut result, level);
                result.push_str(&format!("SELECT({})", count));
            }
            BrainfuckInstruction::Left(count) => {
                indent(&mut result, level);
                result.push_str(&format!("SELECT(-{})", count));
            }
            BrainfuckInstruction::Read => {
                indent(&mut result, level);
                result.push_str(&format!("READ(0)"));
            }
            BrainfuckInstruction::Write => {
                indent(&mut result, level);
                result.push_str(&format!("WRITE(0)"));
            }
            BrainfuckInstruction::Open => {
                indent(&mut result, level);
                result.push_str("OPEN()");
                level += 1;
            }
            BrainfuckInstruction::Close => {
                level -= 1;
                indent(&mut result, level);
                result.push_str("CLOSE()");
            }
            BrainfuckInstruction::Set(value) => {
                indent(&mut result, level);
                result.push_str(&format!("SET(0, {})", value))
            }
            BrainfuckInstruction::ScanLeft => {
                indent(&mut result, level);
                result.push_str("SCAN_LEFT()")
            }
            BrainfuckInstruction::ScanRight => {
                indent(&mut result, level);
                result.push_str("SCAN_RIGHT()")
            }
        }
        result.push('\n');
        index += 1;
    }

    let template = include_str!("template.c");
    template
        .replace("__TAPE_SIZE__", "30000")
        .replace("__CODE__", &result.trim())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::optimizer;
    use crate::parser;

    #[test]
    fn compile_works() {
        let input = optimizer::optimize(
            parser::parse_str(String::from("+++[>+++<-],.[>][<][-]")),
            10,
        );

        let result = compile(input);

        let start = result.find("u8 *dp = tape;\n").unwrap() + 16;
        let end = result.find("__free(tape, tape_size);\n").unwrap() - 6;
        let substring = &result[start..end];
        println!("{}", substring);

        let mut expected = String::new();
        for x in vec![
            "ADJUST(0, 3)",
            "OPEN()",
            "    SELECT(1)",
            "    ADJUST(0, 3)",
            "    SELECT(-1)",
            "    ADJUST(0, -1)",
            "CLOSE()",
            "READ(0)",
            "WRITE(0)",
            "SCAN_RIGHT()",
            "SCAN_LEFT()",
            "SET(0, 0)",
        ] {
            expected.push_str(x);
            expected.push_str("\n    ");
        }

        assert_eq!(substring.trim(), expected.trim_end());
    }
}
