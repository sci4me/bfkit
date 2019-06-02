use crate::parser::BrainfuckInstruction;

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
            },
            BrainfuckInstruction::Sub(count) => {
                indent(&mut result, level);
                result.push_str(&format!("ADJUST(0, -{})", count));
            },
            BrainfuckInstruction::Right(count) => {
                indent(&mut result, level);
                result.push_str(&format!("SELECT({})", count));
            },
            BrainfuckInstruction::Left(count) => {
                indent(&mut result, level);
                result.push_str(&format!("SELECT(-{})", count));
            },
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
            },
            BrainfuckInstruction::Close => {
                level -= 1;
                indent(&mut result, level);
                result.push_str("CLOSE()");
            },
            BrainfuckInstruction::Set(value) => {
                indent(&mut result, level);
                result.push_str(&format!("SET(0, {})", value))
            },
            BrainfuckInstruction::ScanLeft => {
                indent(&mut result, level);
                result.push_str("SCAN_LEFT()")
            },
            BrainfuckInstruction::ScanRight => {
                indent(&mut result, level);
                result.push_str("SCAN_RIGHT()")
            },
        }
        result.push('\n');
        index += 1;
    }

    let template = include_str!("template.c");
    template
        .replace("__TAPE_SIZE__", "30000")
        .replace("__CODE__", &result.trim())
}