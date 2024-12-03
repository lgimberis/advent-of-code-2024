use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn part_one(instructions: String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut sum = 0;
    for (_, [left, right]) in re.captures_iter(instructions.as_str()).map(|c| c.extract()) {
        let left = left.parse::<i32>().expect("Not a number");
        let right = right.parse::<i32>().expect("Not a number");
        sum += left * right;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_given() {
        let instructions =
            String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");
        let result = part_one(instructions);
        assert_eq!(result, 161);
    }
}

fn main() {
    let instructions = read_today_data_file(String::from("03"));
    let part_one_result = part_one(instructions);
    println!("Part one answer: {part_one_result}");
}
