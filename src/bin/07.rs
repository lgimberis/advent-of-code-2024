use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn recursive_explore(target: u64, current: u64, remaining: &[u64]) -> bool {
    if current > target {
        return false;
    }
    if remaining.len() > 0 {
        return recursive_explore(target, current + remaining[0], &remaining[1..])
            || recursive_explore(target, current * remaining[0], &remaining[1..]);
    }
    current == target
}

fn part_one(file: &String) -> u64 {
    let re = Regex::new(r"\d+").unwrap();
    let mut sum = 0;
    for line in file.split("\n") {
        if line.len() == 0 {
            continue;
        }
        let captures: Vec<u64> = re
            .find_iter(line)
            .map(|d| d.as_str().parse::<u64>().unwrap())
            .collect();

        let result = captures[0];
        if recursive_explore(result, captures[1], &captures[2..]) {
            sum += result;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_as_given() {
        let data = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        let result = part_one(&String::from(data));
        assert_eq!(result, 3749);
    }
}

fn main() {
    let file = read_today_data_file(String::from("07"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
}
