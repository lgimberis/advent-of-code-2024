use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<char>> {
    file.split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

fn find_start_and_end(grid: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
    let mut start = (usize::MAX, usize::MAX);
    let mut end = (usize::MAX, usize::MAX);
    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                start = (x, y);
            }
            if c == 'E' {
                end = (x, y);
            }
        }
    }
    (start, end)
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn part_one(file: &String) -> i64 {
    let grid = parse_input(file);
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let (start, end) = find_start_and_end(&grid);
    let mut unmapped_points = VecDeque::new();
    let mut mapped_points = HashSet::new();
    unmapped_points.push_back(((start.0 as i32, start.1 as i32), 0, 0));
    let mut map = HashMap::new();
    let mut it = 0;
    while unmapped_points.len() > 0 && it < 999_999 {
        it += 1;

        let ((x, y), d, score) = unmapped_points.pop_front().unwrap();
        mapped_points.insert((x, y));

        for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            let added_score = 1 + 1000 * std::cmp::min(i.abs_diff(4 - d), i.abs_diff(d));
            if x + dx >= 0
                && x + dx < width
                && y + dy >= 0
                && y + dy < height
                && grid[(y + dy) as usize][(x + dx) as usize] != '#'
                && (!map.contains_key(&(x + dx, y + dy, i))
                    || *map.get(&(x + dx, y + dy, i)).unwrap() > score + added_score)
            {
                map.insert((x + dx, y + dy, i), score + added_score);
                unmapped_points.push_back(((x + dx, y + dy), i, score + added_score));
            }
        }
    }
    if (it == 999_999) {
        println!("Overflowed");
    }
    let mut min_end_score = i64::MAX;
    for d in 0..DIRECTIONS.len() {
        let v = map.get(&(end.0 as i32, end.1 as i32, d));
        match v {
            Some(x) => min_end_score = std::cmp::min(min_end_score, *x as i64),
            None => (),
        }
    }
    min_end_score
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const SECOND_EXAMPLE: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 7036);
    }

    #[test]
    fn test_part_one_second_example() {
        let result = part_one(&String::from(SECOND_EXAMPLE));
        assert_eq!(result, 11048);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("16"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
