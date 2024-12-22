use advent_of_code_2024::read_today_data_file;
use itertools::Itertools;
use std::collections::HashMap;

fn parse_input(file: &String) -> Vec<String> {
    file.lines()
        .filter(|l| l.len() > 0)
        .map(|l| l.to_owned())
        .collect_vec()
}

fn get_initial_movement_vector(required_movement_vector: (isize, isize)) -> Vec<Vec<char>> {
    let horizontal = if required_movement_vector.0 > 0 {
        vec!['>'; required_movement_vector.0 as usize]
    } else {
        vec!['<'; (-required_movement_vector.0) as usize]
    };
    let vertical = if required_movement_vector.1 < 0 {
        vec!['^'; (-required_movement_vector.1) as usize]
    } else {
        vec!['v'; (required_movement_vector.1) as usize]
    };
    vec![
        [vertical.clone(), horizontal.clone()].concat(),
        [horizontal, vertical].concat(),
    ]
}

fn get_instruction_sequences(
    keypad: &HashMap<char, (isize, isize)>,
    desired_output: &String,
) -> Vec<String> {
    // Returns a list of all combinations that generate `desired_output`
    let mut sequence_fragments: Vec<Vec<String>> = Vec::new(); // A list of all permutations at
                                                               // each step
    let mut position = *keypad.get(&'A').unwrap();
    for c in desired_output.chars() {
        let destination = keypad.get(&c).unwrap();
        let required_movement_vector = (destination.0 - position.0, destination.1 - position.1);
        let movements = get_initial_movement_vector(required_movement_vector);
        let fragment = movements
            .iter()
            .cloned()
            .filter(|perm| {
                let mut pos = position;
                for c in perm {
                    match c {
                        '>' => pos = (pos.0 + 1, pos.1),
                        '^' => pos = (pos.0, pos.1 - 1),
                        'v' => pos = (pos.0, pos.1 + 1),
                        '<' => pos = (pos.0 - 1, pos.1),
                        _ => panic!("Unrecognised direction"),
                    };
                    if *keypad.get(&'X').unwrap() == pos {
                        return false;
                    }
                }
                return true;
            })
            .map(|perm| perm.iter().collect::<String>())
            .collect_vec();
        sequence_fragments.push(fragment);
        position = *destination;
    }

    let mut possible_sequences: Vec<String> = vec![String::from("")];
    for fragment in sequence_fragments {
        let mut new_sequences = Vec::new();
        for option in fragment.iter().unique() {
            for sequence in &possible_sequences {
                new_sequences.push(format!("{sequence}{option}A"));
            }
        }
        possible_sequences = new_sequences;
    }
    possible_sequences
}

fn part_one(file: &String) -> u64 {
    let codes = parse_input(file);
    let keypad = HashMap::from([
        ('7', (0, 0)),
        ('8', (1, 0)),
        ('9', (2, 0)),
        ('4', (0, 1)),
        ('5', (1, 1)),
        ('6', (2, 1)),
        ('1', (0, 2)),
        ('2', (1, 2)),
        ('3', (2, 2)),
        ('X', (0, 3)),
        ('0', (1, 3)),
        ('A', (2, 3)),
    ]);

    let robot_keypad = HashMap::from([
        ('X', (0, 0)),
        ('^', (1, 0)),
        ('A', (2, 0)),
        ('<', (0, 1)),
        ('v', (1, 1)),
        ('>', (2, 1)),
    ]);

    let mut complexity = 0;
    for code in codes {
        let first_robot_sequences = get_instruction_sequences(&keypad, &code);
        let mut second_robot_sequences = Vec::new();
        for seq in first_robot_sequences {
            second_robot_sequences.extend(get_instruction_sequences(&robot_keypad, &seq));
        }
        let mut third_robot_sequences = Vec::new();
        for seq in second_robot_sequences {
            third_robot_sequences.extend(get_instruction_sequences(&robot_keypad, &seq));
        }
        let shortest_input_sequence_length =
            third_robot_sequences.iter().fold(usize::MAX, |acc, el| {
                if el.len() < acc {
                    el.len()
                } else {
                    acc
                }
            });
        let numeric_sequence = code[..code.len() - 1].parse::<u64>().unwrap();

        complexity += shortest_input_sequence_length as u64 * numeric_sequence;
    }
    complexity
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 126384);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("21"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
