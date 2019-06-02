use crate::parser::BrainfuckInstruction;

pub fn optimize(ir: Vec<BrainfuckInstruction>, max_passes: u32) -> Vec<BrainfuckInstruction> {
    let opts: Vec<Optimization> = vec![
        clear_loop_removal
    ];

    let mut current = ir;
    let mut last_size = current.len();
    let mut pass = 0;

    while pass < max_passes {
        pass += 1;

        for opt in &opts {
            current = opt(current);
        }

        let len = current.len();
        if len == last_size {
            break;
        } else {
            last_size = len;
        }
    }

    current
}

type Optimization = fn(ir: Vec<BrainfuckInstruction>) -> Vec<BrainfuckInstruction>;

fn clear_loop_removal(ir: Vec<BrainfuckInstruction>) -> Vec<BrainfuckInstruction> {
    fn match_clear(ir: &Vec<BrainfuckInstruction>, index: usize) -> bool {
        if index + 2 >= ir.len() { return false; }

        match (&ir[index], &ir[index + 1], &ir[index + 2]) {
            (BrainfuckInstruction::Open, BrainfuckInstruction::Sub(n), BrainfuckInstruction::Close) if *n == 1 => true,
            _ => false
        }
    }

    let mut result = Vec::new();

    let mut index = 0;
    while index < ir.len() {
        if match_clear(&ir, index) {
            result.push(BrainfuckInstruction::Set(0));
            index += 3;
        } else {
            result.push(ir[index].clone());
            index += 1;
        }
    }

    result
}