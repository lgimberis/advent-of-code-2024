use std::collections::HashSet;

use advent_of_code_2024::read_today_data_file;

fn parse_available_towels(line: &str) -> (HashSet<&str>, usize) {
    let mut patterns = HashSet::new();
    let mut longest = 0usize;
    let available = line.split(",").map(|x| x.trim());
    for towel in available {
        longest = std::cmp::max(longest, towel.len());
        patterns.insert(towel);
    }
    (patterns, longest)
}

fn parse_input(file: &String) -> (HashSet<&str>, usize, Vec<&str>) {
    let mut lines = file.split("\n");
    let (patterns, longest) = parse_available_towels(lines.next().unwrap());
    let designs = lines.filter(|line| line.trim().len() > 0).collect();
    (patterns, longest, designs)
}

fn design_has_matching_pattern(patterns: &HashSet<&str>, longest: usize, design: &str) -> bool {
    if design.len() == 0 {
        return true;
    }
    let furthest_search = std::cmp::min(longest, design.len());
    for search_length in 1..=furthest_search {
        if patterns.contains(&design[0..search_length]) {
            if design_has_matching_pattern(patterns, longest, &design[search_length..]) {
                return true;
            }
        }
    }
    false
}

fn part_one(file: &String) -> u64 {
    let (patterns, longest, designs) = parse_input(file);
    let mut valid_designs = 0u64;
    for design in designs {
        if design_has_matching_pattern(&patterns, longest, design) {
            valid_designs += 1;
        }
    }
    valid_designs
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("19"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
