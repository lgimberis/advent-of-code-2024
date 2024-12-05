use std::collections::HashMap;

use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn build_ordering_ruleset(rules: &Vec<(i32, i32)>) -> HashMap<i32, Vec<i32>> {
    // Because we only need to know if a ruleset is valid,
    // we need a data structure that looks ahead to see if any of its requisites are present,
    // marking it invalid.

    let mut encoded_rules = HashMap::new();
    for (required, follows) in rules {
        if !encoded_rules.contains_key(follows) {
            encoded_rules.insert(*follows, Vec::new());
        }
        encoded_rules
            .entry(*follows)
            .and_modify(|e| e.push(*required));
    }
    encoded_rules
}

fn is_update_valid(update: &Vec<i32>, ruleset: &HashMap<i32, Vec<i32>>) -> bool {
    for (i, v) in update.iter().enumerate() {
        if ruleset.contains_key(v) {
            let rule = ruleset.get(v).unwrap();
            for j in i + 1..update.len() {
                if rule.contains(&update[j]) {
                    return false;
                }
            }
        }
    }
    true
}

fn parse_input(file: &String) -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let rule_re = Regex::new(r"(?P<requires>\d+)\|(?P<follows>\d+)").unwrap();
    let updates_re = Regex::new(r"(\d+,){2,}\d+").unwrap(); // NB assumes at least 3 numbers; not
                                                            // the case in general
    let digits_re = Regex::new(r"\d+").unwrap();

    let rule_captures = rule_re.captures_iter(file);
    let update_captures = updates_re.captures_iter(file);

    for rule in rule_captures {
        let requires = &rule["requires"].parse::<i32>().unwrap();
        let follows = &rule["follows"].parse::<i32>().unwrap();
        rules.push((*requires, *follows));
    }

    for update in update_captures {
        let digit_captures = digits_re.captures_iter(&update[0]);

        let captured: Vec<i32> = digit_captures
            .map(|x| x.extract::<0>())
            .map(|x| x.0.parse::<i32>().unwrap())
            .collect();

        updates.push(captured);
    }
    (rules, updates)
}

fn part_one(file: &String) -> i32 {
    let (rules, updates) = parse_input(file);
    let ruleset = build_ordering_ruleset(&rules);

    let mut median_of_valid_updates = 0;
    for update in updates {
        if is_update_valid(&update, &ruleset) {
            let median_index = update.len() / 2;
            median_of_valid_updates += update[median_index];
        }
    }

    median_of_valid_updates
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_given() {
        let file = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let result = part_one(&file.to_string());
        assert_eq!(result, 143);
    }
}

fn main() {
    let file = read_today_data_file(String::from("05"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
}
