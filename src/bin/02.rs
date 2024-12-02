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

pub fn part_one(reports: &Vec<Vec<u32>>) -> u32 {
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

pub fn part_two(reports: &Vec<Vec<u32>>) -> u32 {
    let mut number_safe = 0;
    'outer: for report in reports {
        if report.len() < 3 {
            if report.len() > 0 {
                number_safe += 1;
            }
            continue;
        };
        let direction_matters = report.len() > 3; // Direction does not matter when the report is
                                                  // of length 3. We can remove any node and still have direction respected.

        fn delta(report: &Vec<u32>, i: usize, problem_index: Option<usize>) -> i32 {
            match problem_index {
                Some(p) => {
                    if i == p {
                        return report[i + 1] as i32 - report[i - 1] as i32;
                    } else if i - 1 == p {
                        return report[i] as i32 - report[i - 2] as i32;
                    } else {
                        return report[i] as i32 - report[i - 1] as i32;
                    }
                }
                None => {
                    return report[i] as i32 - report[i - 1] as i32;
                }
            }
        }
        fn is_delta_increasing(report: &Vec<u32>, i: usize, problem_index: Option<usize>) -> bool {
            return delta(report, i, problem_index) > 0;
        }
        fn is_delta_invalid(
            report: &Vec<u32>,
            i: usize,
            direction: Option<bool>,
            problem_index: Option<usize>,
        ) -> bool {
            let this_delta = delta(report, i, problem_index);
            let direction_factor = match direction {
                Some(d) => d == is_delta_increasing(report, i, problem_index),
                None => true,
            };
            return !direction_factor || this_delta == 0 || this_delta.abs() > 3;
        }

        let mut is_increasing = false;
        let mut problem_index: Option<usize> = None;
        if direction_matters {
            if is_delta_increasing(&report, 1, None) == is_delta_increasing(&report, 2, None) {
                is_increasing = is_delta_increasing(&report, 1, None);
            } else {
                is_increasing = is_delta_increasing(&report, 3, None);
            }
        }
        let mut direction_option = None;
        if direction_matters {
            direction_option = Some(is_increasing);
        }
        for i in 1..report.len() {
            if is_delta_invalid(&report, i, direction_option, problem_index) {
                if problem_index.is_some() {
                    continue 'outer; // This report is invalid; second mistake
                }
                // Try right delta first as left is known to be valid
                problem_index = Some(i);
                if i + 1 < report.len()
                    && is_delta_invalid(&report, i + 1, direction_option, problem_index)
                {
                    // Still invalid - try left
                    problem_index = Some(i - 1);
                    if i > 1 && is_delta_invalid(&report, i - 1, direction_option, problem_index) {
                        continue 'outer; // Unfixable problem
                    }
                }
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
        let result = part_one(&input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_two_basic_remove_second() {
        let input = parse_input(String::from("1 0 4 5"));
        let result = part_two(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_basic_remove_third() {
        let input = parse_input(String::from("3 2 6 1"));
        let result = part_two(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_basic_remove_first() {
        let input = parse_input(String::from("0 4 6 8"));
        let result = part_two(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_basic_remove_last() {
        let input = parse_input(String::from("1 2 3 4 5 6 1"));
        let result = part_two(&input);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_sample() {
        let input = parse_input(TEST_DATA.to_string());
        let result = part_two(&input);
        assert_eq!(result, 4);
    }
}

fn main() {
    let file = read_today_data_file(String::from("02"));
    let parsed = parse_input(file);
    let part_one_result = part_one(&parsed);
    println!("Part one output: {part_one_result}");
    let part_two_result = part_two(&parsed);
    println!("Part two output: {part_two_result}");
}
