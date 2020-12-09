#[derive(Debug, Clone)]
pub struct Instruction {
    operation: String,
    argument: isize,
    hits: usize,
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &str) -> isize {
    let mut boot_code = parse_code(input);
    run_code(&mut boot_code).0
}

#[aoc(day8, part2, brute)]
pub fn solve_part2_brute(input: &str) -> isize {
    let mut boot_code = parse_code(input);
    fix_code(&mut boot_code)
}

fn parse_code(code: &str) -> Vec<Instruction> {
    let mut result: Vec<Instruction> = Vec::new();

    for line in code.lines() {
        let instruction = line.split_whitespace().collect::<Vec<&str>>();
        let inst = Instruction {
            operation: instruction[0].to_string(),
            argument: instruction[1].parse().unwrap(),
            hits: 0,
        };
        result.push(inst);
    }
    result
}

fn run_code(code: &mut Vec<Instruction>) -> (isize, bool) {
    let mut accumulator = 0;
    let mut pc = 0;
    let code_length = code.len();
    let mut loop_detected = false;

    loop {
        // check for end of code
        if pc >= code_length {
            break;
        }

        // check for loop
        if code[pc].hits > 0 {
            loop_detected = true;
            break;
        }
        code[pc].hits += 1;
        match code[pc].operation.as_str() {
            "acc" => {
                accumulator += code[pc].argument;
                pc += 1;
            }
            "jmp" => pc = (pc as isize + code[pc].argument) as usize,
            "nop" => pc += 1,
            _ => panic!("Unknown operation"),
        }
    }
    (accumulator, loop_detected)
}

fn fix_code(code: &mut Vec<Instruction>) -> isize {
    let mut flip_candidates: Vec<usize> = Vec::new();

    for (index, inst) in code.iter().enumerate() {
        match inst.operation.as_str() {
            "jmp" | "nop" => flip_candidates.push(index),
            _ => {}
        }
    }

    for flip_index in flip_candidates {
        let mut c = code.clone();
        match c[flip_index].operation.as_str() {
            "jmp" => c[flip_index].operation = String::from("nop"),
            "nop" => c[flip_index].operation = String::from("jmp"),
            _ => {}
        }

        let (accumulator, loop_detected) = run_code(&mut c);

        if !loop_detected {
            return accumulator;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_CODE: &str =
        "nop +0\nacc +1\njmp +4\nacc +3\njmp -3\nacc -99\nacc +1\njmp -4\nacc +6";

    #[test]
    fn accumulator_with_loop() {
        let mut boot_code = parse_code(EXAMPLE_CODE);

        assert!(run_code(&mut boot_code).0 == 5);
    }

    #[test]
    fn accumulator_with_fixed_code() {
        let mut boot_code = parse_code(EXAMPLE_CODE);

        assert!(fix_code(&mut boot_code) == 8);
    }
}
