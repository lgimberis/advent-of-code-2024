use advent_of_code_2024::read_today_data_file;
use regex::Regex;
use std::collections::HashMap;

fn sort_input_arrays(content: &String) -> (Vec<u32>, Vec<u32>) {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    let re = Regex::new(r"^\s*(?P<left>\d+)\s+(?P<right>\d+)$").unwrap();

    let lines: Vec<&str> = content.split("\n").collect();
    for s in &lines {
        match re.captures(s) {
            None => {}
            Some(captures) => {
                let left = &captures["left"].parse::<u32>().expect("not a number");
                let right = &captures["right"].parse::<u32>().expect("not a number");
                first.push(*left);
                second.push(*right);
            }
        }
    }

    // Sort both arrays
    first.sort();
    second.sort();

    (first, second)
}

pub fn part_one(content: &String) -> u32 {
    let (first, second) = sort_input_arrays(content);
    let mut sum_of_differences: u32 = 0;
    for i in 0..first.len() {
        sum_of_differences += first[i].abs_diff(second[i]);
    }

    sum_of_differences
}

pub fn part_two(content: &String) -> u32 {
    let (first, second) = sort_input_arrays(content);

    // Construct hash maps of both arrays
    let mut first_map = HashMap::new();
    let mut second_map = HashMap::new();
    for i in first {
        let count = first_map.entry(i).or_insert(0);
        *count += 1;
    }
    for i in second {
        let count = second_map.entry(i).or_insert(0);
        *count += 1;
    }

    let mut similarity_score = 0;
    for (key, value) in &first_map {
        if second_map.contains_key(key) {
            similarity_score += *key * *value * *second_map.entry(*key).or_insert(0);
        }
    }

    similarity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let test_data = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        let result = part_one(&test_data.to_string());
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let test_data = "3   4
        4   3
        2   5
        1   3
        3   9
        3   3";

        let result = part_two(&test_data.to_string());
        assert_eq!(result, 31);
    }
}

fn main() {
    let content = read_today_data_file("01".to_string());
    let sum_of_differences = part_one(&content);
    println!("Part one: Sum of differences = {sum_of_differences}");
    let similarity_score = part_two(&content);
    println!("Part two: Similarity score = {similarity_score}");
}
