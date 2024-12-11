use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn parse_input(file: &String) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(file)
        .into_iter()
        .map(|x| x[0].to_string().parse().unwrap())
        .collect()
}

fn part_one(file: &String, blinks: u64) -> i64 {
    let mut parsed_input = parse_input(file);
    for _i in 0..blinks {
        // We don't actually seem to care about stone order in this one
        for j in 0..parsed_input.len() {
            let v = parsed_input[j];
            if v.to_string().len() % 2 == 0 {
                let s = v.to_string();
                let midpoint = s.len() / 2;
                let as_vec = s.chars().collect::<Vec<char>>();
                parsed_input[j] = as_vec[..midpoint]
                    .into_iter()
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap();
                parsed_input.push(
                    as_vec[midpoint..]
                        .into_iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap(),
                );
                continue;
            }
            if v == 0 {
                parsed_input[j] = 1;
                continue;
            }
            parsed_input[j] *= 2024;
        }
    }
    parsed_input.len() as i64
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "125 17";

    #[test]
    fn test_part_one_one_blink() {
        let result = part_one(&String::from(EXAMPLE_DATA), 1);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_one_six_blinks() {
        let result = part_one(&String::from(EXAMPLE_DATA), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA), 25);
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("11"));
    let part_one_result = part_one(&file, 25);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
