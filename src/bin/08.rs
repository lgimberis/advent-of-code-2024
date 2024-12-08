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

fn find_antinodes(a1: (i32, i32), a2: (i32, i32)) -> (Option<(i32, i32)>, Option<(i32, i32)>) {
    // Core idea: When nodes are seperated by dx in x and dy in y, the antinodes are located at
    // (x1 - dx, y1 - dy) and (x2 + dx, y2 + dy)
    let dx = a2.0 - a1.0;
    let dy = a2.1 - a1.1;
    let antinode1 = if a1.0 < dx || a1.1 < dy {
        None
    } else {
        Some((a1.0 - dx, a1.1 - dy))
    };
    let antinode2 = if a2.0 < -dx || a1.1 < -dy {
        None
    } else {
        Some((a2.0 + dx, a2.1 + dy))
    };
    (antinode1, antinode2)
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let height = parsed_input.len();
    let width = parsed_input[0].len(); // Assumed constant
    let antenna_map = find_antennas(parsed_input);
    let mut map = vec![vec![false; width]; height];

    for (_c, antenna_locations) in antenna_map {
        for (i, loc1) in antenna_locations.iter().enumerate() {
            for loc2 in &antenna_locations[i + 1..] {
                let (antinode1, antinode2) = find_antinodes(*loc1, *loc2);
                if antinode1.is_some() {
                    let (x, y) = antinode1.unwrap();
                    if x < width as i32 && y < height as i32 {
                        map[x as usize][y as usize] = true;
                    }
                }
                if antinode2.is_some() {
                    let (x, y) = antinode2.unwrap();
                    if x < width as i32 && y < height as i32 {
                        map[x as usize][y as usize] = true;
                    }
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

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    0
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
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("08"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
