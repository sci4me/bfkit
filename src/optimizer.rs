use crate::parser::BrainfuckInstruction;

/// Performs up to `max_passes` optimization passes on a sequence of BrainfuckInstructions.
/// Will stop early, before `max_passes`, if no progress is being made.
pub fn optimize(ir: Vec<BrainfuckInstruction>, max_passes: u32) -> Vec<BrainfuckInstruction> {
    let opts: Vec<Optimization> = vec![contraction, clear_loop_removal, scan_loop_removal];

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
        if index + 2 >= ir.len() {
            return false;
        }

        match (&ir[index], &ir[index + 1], &ir[index + 2]) {
            (
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Sub(n),
                BrainfuckInstruction::Close,
            ) if *n == 1 => true,
            _ => false,
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

macro_rules! generate_contraction {
    ( $( $name:ident ),* ) => {
        fn contraction(ir: Vec<BrainfuckInstruction>) -> Vec<BrainfuckInstruction> {
            let mut result = Vec::new();

            let mut index = 0;
            while index < ir.len() {
                match &ir[index] {
                    $(
                        BrainfuckInstruction::$name(n) => {
                            let mut count = *n;
                            while index + 1 < ir.len() {
                                if let BrainfuckInstruction::$name(x) = &ir[index + 1] {
                                    count = count.wrapping_add(*x);
                                    index += 1;
                                } else {
                                    break;
                                }
                            }

                            result.push(BrainfuckInstruction::$name(count));
                            index += 1;
                        }
                    )*,
                    _ => {
                        result.push(ir[index].clone());
                        index += 1;
                    }
                }
            }

            result
        }
    }
}

generate_contraction!(Add, Sub, Right, Left);

fn scan_loop_removal(ir: Vec<BrainfuckInstruction>) -> Vec<BrainfuckInstruction> {
    fn match_scan_loop(ir: &Vec<BrainfuckInstruction>, index: usize) -> bool {
        if index + 2 >= ir.len() {
            return false;
        }

        match (&ir[index], &ir[index + 1], &ir[index + 2]) {
            (
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Left(n),
                BrainfuckInstruction::Close,
            ) if *n == 1 => true,
            (
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Right(n),
                BrainfuckInstruction::Close,
            ) if *n == 1 => true,
            _ => false,
        }
    }

    let mut result = Vec::new();

    let mut index = 0;
    while index < ir.len() {
        if match_scan_loop(&ir, index) {
            if let BrainfuckInstruction::Left(_) = &ir[index + 1] {
                result.push(BrainfuckInstruction::ScanLeft);
            } else if let BrainfuckInstruction::Right(_) = &ir[index + 1] {
                result.push(BrainfuckInstruction::ScanRight);
            } else {
                unreachable!();
            }

            index += 3;
        } else {
            result.push(ir[index].clone());
            index += 1;
        }
    }

    result
}
