use advent_of_code_2024::read_today_data_file;
use itertools::Itertools;

fn parse_input(file: &String) -> (Vec<Vec<u64>>, Vec<Vec<u64>>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for spec in file.split("\n\n") {
        let mut is_lock = false;
        let mut vec = Vec::new();
        for (y, line) in spec.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let value = if c == '#' { 1 } else { 0 };
                if y == 0 {
                    vec.push(value);
                } else {
                    vec[x] += value;
                }
            }
            if y == 0 {
                if vec.iter().all(|v| *v == 1) {
                    is_lock = true;
                }
            }
        }
        vec = vec.into_iter().map(|x| x - 1).collect_vec();
        if is_lock {
            locks.push(vec);
        } else {
            keys.push(vec);
        }
    }
    (locks, keys)
}

fn does_key_fit_lock(key: &Vec<u64>, lock: &Vec<u64>) -> bool {
    key.iter().zip(lock.iter()).all(|(x, y)| x + y <= 5)
}

fn part_one(file: &String) -> u64 {
    let (locks, keys) = parse_input(file);
    let mut count = 0;
    for lock in &locks {
        for key in &keys {
            if does_key_fit_lock(key, lock) {
                count += 1;
            }
        }
    }
    count
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("25"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
