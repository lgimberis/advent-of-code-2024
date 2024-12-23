use advent_of_code_2024::read_today_data_file;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

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
    for _i in 0..n {
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

fn generate_deltas(secret: u64, n: u64) -> Vec<(u64, i64)> {
    let mut previous = secret;
    let mut v = Vec::new();
    for _i in 0..n {
        let s = next_secret(previous);
        let price = s % 10;
        let previous_price = (previous % 10) as i64;
        v.push((price % 10, price as i64 - previous_price));
        previous = s;
    }
    v
}
fn part_two(file: &String) -> u64 {
    let secret_numbers = parse_input(file);
    let mut sequence_earnings = HashMap::new();
    for secret in secret_numbers {
        let deltas = generate_deltas(secret, 2000);
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        for (price, delta) in deltas {
            queue.push_back(delta);
            if queue.len() < 4 {
                continue;
            }
            let key = queue[0] * 8000 + queue[1] * 400 + queue[2] * 20 + queue[3];
            if seen.contains(&key) {
                queue.pop_front();
                continue;
            }
            let e: &mut u64 = sequence_earnings.entry(key.clone()).or_default();
            *e += price;
            seen.insert(key);
            queue.pop_front();
        }
    }
    sequence_earnings
        .iter()
        .fold(0, |acc, (_key, &total)| std::cmp::max(acc, total))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "1
10
100
2024";

    const SECOND_EXAMPLE: &str = "1
2
3
2024";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 37327623);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(SECOND_EXAMPLE));
        assert_eq!(result, 23);
    }
}

fn main() {
    let file = read_today_data_file(String::from("22"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
