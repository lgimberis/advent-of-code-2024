use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn parse_input(data: String) -> Vec<Vec<u32>> {
    let lines: Vec<&str> = data.split("\n").collect();

    let re = Regex::new(r"\d+").unwrap();
    let mut reports = Vec::new();

    for line in &lines {
        let report = re
            .find_iter(line)
            .map(|i| i.as_str().parse::<u32>().expect("number was not a number"))
            .collect();
        reports.push(report);
    }

    reports
}

pub fn part_one(reports: Vec<Vec<u32>>) -> u32 {
    let mut number_safe = 0;
    'outer: for report in reports {
        if report.len() < 1 {
            continue;
        };
        let is_increasing = report[1] > report[0];
        let mut prev = report[0];
        for level in &report[1..] {
            let diff = level.abs_diff(prev);
            let has_increased = *level > prev;
            prev = *level;
            if is_increasing != has_increased {
                continue 'outer;
            }
            if diff == 0 || diff > 3 {
                continue 'outer;
            }
        }
        number_safe += 1;
    }
    number_safe
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9";

    #[test]
    fn test_part_one() {
        let input = parse_input(TEST_DATA.to_string());
        let result = part_one(input);
        assert_eq!(result, 2);
    }
}

fn main() {
    let file = read_today_data_file(String::from("02"));
    let part_one_result = part_one(parse_input(file));
    println!("Part one output: {part_one_result}");
}
