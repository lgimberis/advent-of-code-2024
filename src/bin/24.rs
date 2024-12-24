use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use advent_of_code_2024::read_today_data_file;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug)]
struct GateConnection {
    first: String,
    second: String,
    operation: Operation,
    out: String,
}

fn parse_input(file: &String) -> (HashMap<String, u64>, Vec<GateConnection>) {
    let mut operations = Vec::new();
    let mut wires = HashMap::new();
    let initial_value_re = Regex::new(r"^([xy]\d{2}): ([01])$").unwrap();
    let operation_re = Regex::new(r"^(\w+) (\w+) (\w+).*?(\w+)$").unwrap();

    let mut finished_initial_values = false;
    for line in file.lines() {
        if line.is_empty() {
            finished_initial_values = true;
            continue;
        }
        if !finished_initial_values {
            let cap = initial_value_re.captures(line).unwrap();
            wires.insert(cap[1].to_string(), cap[2].parse::<u64>().unwrap());
        } else {
            let cap = operation_re.captures(line).unwrap();
            let operation = match &cap[2] {
                "AND" => Operation::AND,
                "OR" => Operation::OR,
                "XOR" => Operation::XOR,
                d => panic!("Unrecognised operation {d}"),
            };
            operations.push(GateConnection {
                first: cap[1].to_owned(),
                second: cap[3].to_owned(),
                operation,
                out: cap[4].to_owned(),
            });
        }
    }
    (wires, operations)
}

fn calculate_value(wires: &HashMap<String, u64>, operation: &GateConnection) -> Option<u64> {
    let first = wires.get(&operation.first)?;
    let second = wires.get(&operation.second)?;
    Some(match operation.operation {
        Operation::OR => first | second,
        Operation::AND => first & second,
        Operation::XOR => first ^ second,
    })
}

fn part_one(file: &String) -> u64 {
    let (mut wires, mut operations) = parse_input(file);

    'outer: while operations.len() > 0 {
        for (i, operation) in operations.iter().enumerate() {
            if let Some(value) = calculate_value(&wires, &operation) {
                wires.insert(operation.out.clone(), value);
                operations.remove(i);
                continue 'outer;
            }
        }
        println!("Nothing matched!");
        break 'outer;
    }
    let mut z = 0u64;
    for (key, value) in wires {
        if key.starts_with("z") && value > 0 {
            let bits = key[1..].parse::<u64>().unwrap();
            z += 1 << bits;
        }
    }
    z
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";

    const EXAMPLE_DATA: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&String::from(SMALL_EXAMPLE));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 2024);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("24"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
