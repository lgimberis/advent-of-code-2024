use std::collections::VecDeque;

use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn parse_input(file: &String) -> (Vec<u64>, Vec<u8>) {
    let mut finished_registers = false;
    let mut registers = Vec::new();
    let mut program = Vec::new();
    let digit_re = Regex::new(r"\d+").unwrap();
    for line in file.split("\n") {
        if line.len() == 0 {
            finished_registers = true;
        } else if !finished_registers {
            let m = digit_re.captures(line).unwrap()[0].parse::<u64>().unwrap();
            registers.push(m);
        } else {
            for digit in digit_re.captures_iter(line) {
                program.push(digit[0].parse::<u8>().unwrap());
            }
        }
    }
    (registers, program)
}

fn combo_operator(registers: &Vec<u64>, operand: u8) -> u64 {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        d => {
            if d <= 3 {
                return operand as u64;
            }
            panic!("Value {operand} assessed as combo operator");
        }
    }
}

fn process_instruction(
    registers: &mut Vec<u64>,
    opcode: u8,
    operand: u8,
    instruction_pointer: &mut usize,
    outputs: &mut Vec<u64>,
) {
    match opcode {
        0 => registers[0] /= (2u64.pow(combo_operator(registers, operand) as u32)),
        1 => registers[1] = registers[1] ^ operand as u64,
        2 => registers[1] = combo_operator(registers, operand) % 8,
        3 => {
            if registers[0] != 0 {
                *instruction_pointer = operand as usize
            } else {
                *instruction_pointer += 2;
            }
        }
        4 => registers[1] = registers[1] ^ registers[2],
        5 => outputs.push(combo_operator(registers, operand) % 8),
        6 => registers[1] = registers[0] / 2u64.pow(combo_operator(registers, operand) as u32),
        7 => registers[2] = registers[0] / 2u64.pow(combo_operator(registers, operand) as u32),
        d => panic!("Unrecognised register value {d}"),
    }
    if opcode != 3 {
        *instruction_pointer += 2;
    }
}

fn part_one(file: &String) -> String {
    let (mut registers, program) = parse_input(file);
    let mut it = 0;
    const MAX_IT: u64 = 999_999;
    let mut outputs: Vec<u64> = Vec::new();

    let mut instruction_pointer = 0;
    while it < MAX_IT && instruction_pointer <= program.len() - 1 {
        it += 1;

        process_instruction(
            &mut registers,
            program[instruction_pointer],
            program[instruction_pointer + 1],
            &mut instruction_pointer,
            &mut outputs,
        );
    }
    if it == MAX_IT {
        println!("Overflowed");
    }
    outputs
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn part_two(file: &String) -> u64 {
    // Yeah, so I'm not currently competent enough to make a full general solution, so...
    // This will be horribly specific to my input data following these assumptions
    // 1: In all cases, Register A evolves by being divided ONCE by exactly 8 with each program loop
    // 2: There is a single jump at the end, which goes back to the very start and terminates when
    //    A is 0
    // 3: Each loop can be solved by finding A modulo 8 ONLY
    // 4: There is only one output per loop
    // 5: There is no persistence in B and C between loops; they are recalculated from A each loop
    // 6: The division by 8 occurs at the END of each loop, before the 0-check
    // If any of these aren't true, this "solution" will not work
    //
    // Anyway,
    // From this, defining the nth loop's initial A register as A(n), A(n) = 8A(n + 1) + k(n), where
    // k(n) is in 0..=7
    // Our expected return value is then simply A(0). We have the upper bound A(0) <
    // 8^N because the final output requires that A(N - 1) < 8.
    //
    // We can just try each of the possible 8 values of k(n) for each loop and see which gives us our desired output
    // In fact, we can almost memoize a table for Output->k(n), but C(n) depends on A(n)'s
    // magnitude :(

    let (_registers, program) = parse_input(file);

    fn get_output(program: &Vec<u8>, a: u64) -> u64 {
        let mut registers: Vec<u64> = vec![a, 0, 0]; // Assume C unchanged or reset
        let mut instruction_pointer = 0;
        let mut outputs = Vec::new();
        for _i in 0..program.len() / 2 {
            process_instruction(
                &mut registers,
                program[instruction_pointer],
                program[instruction_pointer + 1],
                &mut instruction_pointer, // Assumption 2
                &mut outputs,
            );
        }
        outputs[0]
    }

    let mut constants: Vec<VecDeque<u64>> = Vec::new();
    while constants.len() < program.len() {
        if constants.len() > 0 && constants[constants.len() - 1].len() == 0 {
            constants.pop();
            let l = constants.len();
            constants[l - 1].pop_front();
            continue;
        }
        let output = program[program.len() - constants.len() - 1];
        let mut out = 0u64;
        for v in &constants {
            out = 8 * out + v[0];
        }
        out *= 8; // "Undo" the division that is done at the end of the line, not the start
        let mut this_tier = VecDeque::new();
        for b in 0..8 {
            if get_output(&program, out + b) == output as u64 {
                this_tier.push_back(b);
            }
        }
        if this_tier.len() > 0 {
            constants.push(this_tier);
        } else {
            let l = constants.len();
            constants[l - 1].pop_front();
        }
    }
    let mut out = 0u64;
    for v in constants {
        out = 8 * out + v[0];
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const PART_TWO_EXAMPLE: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(PART_TWO_EXAMPLE));
        assert_eq!(result, 117440);
    }
}

fn main() {
    let file = read_today_data_file(String::from("17"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
