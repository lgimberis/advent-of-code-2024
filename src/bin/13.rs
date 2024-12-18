use advent_of_code_2024::read_today_data_file;
use regex::CaptureMatches;
use regex::Regex;
fn parse_input(file: &String, offset: u64) -> Vec<Vec<(u64, u64)>> {
    let lines: Vec<&str> = file.split("\n").filter(|x| x.trim().len() > 0).collect();
    let mut machines = Vec::new();
    let re = Regex::new(r"\d+").unwrap();

    fn process(captures: CaptureMatches) -> (u64, u64) {
        let vec = captures
            .map(|x| x.extract::<0>().0.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        (vec[0], vec[1])
    }

    for machine_i in 0..lines.len() / 3 {
        machines.push(Vec::new());
        let button_a = process(re.captures_iter(lines[3 * machine_i]));
        let button_b = process(re.captures_iter(lines[3 * machine_i + 1]));
        let prize = process(re.captures_iter(lines[3 * machine_i + 2]));
        machines[machine_i].push(button_a);
        machines[machine_i].push(button_b);
        machines[machine_i].push((prize.0 + offset, prize.1 + offset));
    }
    machines
}

fn integer_solutions(
    target: (u64, u64),
    cheap: (u64, u64),
    expensive: (u64, u64),
) -> Option<(u64, u64)> {
    // Returns integer numbers to be multiplied by arguments cheap and expensive respectively to get
    // target, in both of their values

    let target = (target.0 as i64, target.1 as i64);
    let cheap = (cheap.0 as i64, cheap.1 as i64);
    let expensive = (expensive.0 as i64, expensive.1 as i64);
    let n_expensive =
        (target.1 * cheap.0 - target.0 * cheap.1) / (cheap.0 * expensive.1 - cheap.1 * expensive.0);
    let n_cheap = (target.0 - n_expensive * expensive.0) / cheap.0;
    if n_cheap * cheap.0 + n_expensive * expensive.0 != target.0
        || n_cheap * cheap.1 + n_expensive * expensive.1 != target.1
    {
        return None;
    }
    return Some((n_cheap as u64, n_expensive as u64));
}

fn part_one(file: &String) -> u64 {
    let machines = parse_input(file, 0);

    let mut winning_token_costs = 0;
    for machine in machines {
        let multipliers = integer_solutions(machine[2], machine[1], machine[0]);
        if multipliers.is_some() {
            winning_token_costs += multipliers.unwrap().0 + multipliers.unwrap().1 * 3;
        }
    }
    winning_token_costs
}

fn part_two(file: &String) -> u64 {
    let machines = parse_input(file, 10_000_000_000_000u64);

    let mut winning_token_costs = 0;
    for machine in machines {
        let multipliers = integer_solutions(machine[2], machine[1], machine[0]);
        if multipliers.is_some() {
            winning_token_costs += multipliers.unwrap().0 + multipliers.unwrap().1 * 3;
        }
    }
    winning_token_costs
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 480);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("13"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
