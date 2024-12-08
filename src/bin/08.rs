use advent_of_code_2024::read_today_data_file;
use std::collections::HashMap;

fn parse_input(file: &String) -> Vec<Vec<char>> {
    file.split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn find_antennas(map: Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut result = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == '.' {
                continue;
            }
            let v = result.entry(c).or_insert(Vec::new());
            v.push((x as i32, y as i32));
        }
    }
    result
}

fn find_antinode_single(a1: (i32, i32), a2: (i32, i32), bounds: (i32, i32)) -> Vec<(i32, i32)> {
    // Core idea: When nodes are seperated by dx in x and dy in y, the antinodes are located at
    // (x1 - dx, y1 - dy) and (x2 + dx, y2 + dy)
    let mut result = Vec::new();
    let dx = a2.0 - a1.0;
    let dy = a2.1 - a1.1;
    if a1.0 - dx >= 0 && a1.1 - dy >= 0 && a1.0 - dx < bounds.0 && a1.1 - dy < bounds.1 {
        result.push((a1.0 - dx, a1.1 - dy));
    }
    if a2.0 + dx >= 0 && a2.1 + dy >= 0 && a2.0 + dx < bounds.0 && a2.1 + dy < bounds.1 {
        result.push((a2.0 + dx, a2.1 + dy));
    }
    result
}

fn find_antinodes_multiple(a1: (i32, i32), a2: (i32, i32), bounds: (i32, i32)) -> Vec<(i32, i32)> {
    // Core idea: When nodes are seperated by dx in x and dy in y, the antinodes are located at
    // (x1 - dx, y1 - dy) and (x2 + dx, y2 + dy)
    let mut result = Vec::new();
    let dx = a2.0 - a1.0;
    let dy = a2.1 - a1.1;
    let mut i = 0;
    while a1.0 - i * dx >= 0
        && a1.1 - i * dy >= 0
        && a1.0 - i * dx < bounds.0
        && a1.1 - i * dy < bounds.1
    {
        result.push((a1.0 - i * dx, a1.1 - i * dy));
        i += 1;
    }
    i = 0;
    while a2.0 + i * dx >= 0
        && a2.1 + i * dy >= 0
        && a2.0 + i * dx < bounds.0
        && a2.1 + i * dy < bounds.1
    {
        result.push((a2.0 + i * dx, a2.1 + i * dy));
        i += 1;
    }
    result
}

fn either_part(
    file: &String,
    find_antinodes: fn((i32, i32), (i32, i32), (i32, i32)) -> Vec<(i32, i32)>,
) -> i64 {
    let parsed_input = parse_input(file);
    let height = parsed_input.len();
    let width = parsed_input[0].len(); // Assumed constant
    let antenna_map = find_antennas(parsed_input);
    let mut map = vec![vec![false; width]; height];

    for (_c, antenna_locations) in antenna_map {
        for (i, loc1) in antenna_locations.iter().enumerate() {
            for loc2 in &antenna_locations[i + 1..] {
                for (x, y) in find_antinodes(*loc1, *loc2, (width as i32, height as i32)) {
                    map[x as usize][y as usize] = true;
                }
            }
        }
    }

    let mut antinodes = 0;
    for row in map {
        for b in row {
            if b {
                antinodes += 1;
            }
        }
    }
    antinodes
}

fn part_one(file: &String) -> i64 {
    either_part(file, find_antinode_single)
}

fn part_two(file: &String) -> i64 {
    either_part(file, find_antinodes_multiple)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 14);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 34);
    }
}

fn main() {
    let file = read_today_data_file(String::from("08"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
