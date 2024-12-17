use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn parse_input(file: &String) -> (Vec<u32>, Vec<u8>) {
    let mut finished_registers = false;
    let mut registers = Vec::new();
    let mut program = Vec::new();
    let digit_re = Regex::new(r"\d+").unwrap();
    for line in file.split("\n") {
        if line.len() == 0 {
            finished_registers = true;
        } else if !finished_registers {
            let m = digit_re.captures(line).unwrap()[0].parse::<u32>().unwrap();
            registers.push(m);
        } else {
            for digit in digit_re.captures_iter(line) {
                program.push(digit[0].parse::<u8>().unwrap());
            }
        }
    }
    (registers, program)
}

fn combo_operator(registers: &Vec<u32>, operand: u8) -> u32 {
    match operand {
        4 => registers[0],
        5 => registers[1],
        6 => registers[2],
        d => {
            if d <= 3 {
                return operand as u32;
            }
            panic!("Value {operand} assessed as combo operator");
        }
    }
}

fn process_instruction(
    registers: &mut Vec<u32>,
    opcode: u8,
    operand: u8,
    instruction_pointer: &mut usize,
    outputs: &mut Vec<u32>,
) {
    match opcode {
        0 => registers[0] /= 2u32.pow(combo_operator(registers, operand)),
        1 => registers[1] = registers[1] ^ operand as u32,
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
        6 => registers[1] = registers[0] / 2u32.pow(combo_operator(registers, operand)),
        7 => registers[2] = registers[0] / 2u32.pow(combo_operator(registers, operand)),
        d => panic!("Unrecognised register value {d}"),
    }
    if opcode != 3 {
        *instruction_pointer += 2;
    }
}

fn part_one(file: &String) -> String {
    let (mut registers, program) = parse_input(file);
    println!("{:?}\n{:?}", registers, program);
    let mut it = 0;
    const MAX_IT: u32 = 999_999;
    let mut outputs: Vec<u32> = Vec::new();

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
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("17"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
