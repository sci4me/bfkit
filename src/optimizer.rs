use crate::parser::BrainfuckInstruction;

/// Performs up to `max_passes` optimization passes on a sequence of BrainfuckInstructions.
/// Will stop early, before `max_passes`, if no progress is being made.
pub fn optimize(ir: Vec<BrainfuckInstruction>, max_passes: u32) -> Vec<BrainfuckInstruction> {
    let opts: Vec<Optimization> = vec![
        dead_code_removal,
        contraction,
        clear_loop_removal,
        scan_loop_removal,
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

fn dead_code_removal(ir: Vec<BrainfuckInstruction>) -> Vec<BrainfuckInstruction> {
    if let BrainfuckInstruction::Open = ir[0] {
        let mut index = 1;
        let mut level = 1;

        while index < ir.len() && level > 0 {
            if let BrainfuckInstruction::Open = ir[index] {
                level += 1;
            } else if let BrainfuckInstruction::Close = ir[index] {
                level -= 1;
            }
            index += 1;
        }

        ir[index..].to_vec()
    } else {
        ir
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::parse_str;

    #[test]
    fn optimize_works() {
        let input = parse_str(String::from("[lol]+++[>+++<-][-][>]+[<]"));
        let len = input.len();

        let result = optimize(input, 10);

        assert_eq!(
            result,
            vec![
                BrainfuckInstruction::Add(3),
                BrainfuckInstruction::Open,
                BrainfuckInstruction::Right(1),
                BrainfuckInstruction::Add(3),
                BrainfuckInstruction::Left(1),
                BrainfuckInstruction::Sub(1),
                BrainfuckInstruction::Close,
                BrainfuckInstruction::Set(0),
                BrainfuckInstruction::ScanRight,
                BrainfuckInstruction::Add(1),
                BrainfuckInstruction::ScanLeft
            ]
        );
        assert!(result.len() < len);
    }

    #[test]
    fn dead_code_removal_works() {
        let input = parse_str(String::from("[++[>+<-]]++-"));
        let len = input.len();

        let result = dead_code_removal(input);

        assert_eq!(result, parse_str(String::from("++-")));
        assert!(result.len() < len);
    }

    #[test]
    fn clear_loop_removal_works() {
        let input = parse_str(String::from("[-]"));
        let len = input.len();

        let result = clear_loop_removal(input);

        assert_eq!(result, vec![BrainfuckInstruction::Set(0)]);
        assert!(result.len() < len);
    }

    #[test]
    fn contraction_works() {
        let input = parse_str(String::from("++--->>>><<<<<"));
        let len = input.len();

        let result = contraction(input);

        assert_eq!(
            result,
            vec![
                BrainfuckInstruction::Add(2),
                BrainfuckInstruction::Sub(3),
                BrainfuckInstruction::Right(4),
                BrainfuckInstruction::Left(5)
            ]
        );
        assert!(result.len() < len);
    }

    #[test]
    fn scan_loop_removal_works() {
        let input = parse_str(String::from("[>][<]"));
        let len = input.len();

        let result = scan_loop_removal(input);

        assert_eq!(
            result,
            vec![
                BrainfuckInstruction::ScanRight,
                BrainfuckInstruction::ScanLeft
            ]
        );
        assert!(result.len() < len);
    }
}
