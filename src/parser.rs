#[derive(Debug, Clone)]
pub enum BrainfuckInstruction {
    Add(u8),
    Sub(u8),
    Right(usize),
    Left(usize),
    Read,
    Write,
    Open,
    Close,
    Set(u8),
    ScanLeft,
    ScanRight
}

pub fn parse_str(code: String) -> Vec<BrainfuckInstruction> {
    parse(&code.chars().collect::<Vec<char>>())
}

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
            _ => {
            }
        }
    }

    result
}