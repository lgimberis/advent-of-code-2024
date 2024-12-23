use advent_of_code_2024::read_today_data_file;
use itertools::Itertools;

fn parse_input(file: &String) -> Vec<u64> {
    file.lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<u64>().unwrap())
        .collect_vec()
}

const MOD: u64 = 16777216u64;

fn next_secret(secret: u64) -> u64 {
    let first = ((secret << 6) ^ secret) % MOD;
    let second = ((first >> 5) ^ first) % MOD;
    ((second << 11) ^ second) % MOD
}

fn generate_nth_secret(secret: u64, n: u64) -> u64 {
    let mut s = secret;
    for i in 0..n {
        s = next_secret(s);
    }
    s
}

fn part_one(file: &String) -> u64 {
    let secret_numbers = parse_input(file);
    secret_numbers
        .iter()
        .map(|&d| generate_nth_secret(d, 2000))
        .sum()
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "1
10
100
2024";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("22"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
