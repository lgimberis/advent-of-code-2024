use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn part_one(content: &String) -> u32 {
    let mut first: Vec<u32> = Vec::new();
    let mut second: Vec<u32> = Vec::new();
    let re = Regex::new(r"^(?P<left>\d+)\s+(?P<right>\d+)$").unwrap();

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

    let mut sum_of_differences: u32 = 0;
    for i in 0..first.len() {
        sum_of_differences += first[i].abs_diff(second[i]);
    }

    sum_of_differences
}

fn main() {
    let content = read_today_data_file("01".to_string());
    let sum_of_differences = part_one(&content);
    println!("{sum_of_differences}");
}
