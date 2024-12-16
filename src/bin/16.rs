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

fn score_cost_turning(d: usize, other: usize) -> i32 {
    1000 * std::cmp::min(other.abs_diff(4 - d), other.abs_diff(d)) as i32
}

fn generate_map(grid: Vec<Vec<char>>) -> HashMap<(i32, i32), Vec<(usize, i32)>> {
    let width = grid[0].len() as i32;
    let height = grid.len() as i32;
    let (start, _end) = find_start_and_end(&grid);
    let mut unmapped_points = VecDeque::new();
    unmapped_points.push_back(((start.0 as i32, start.1 as i32), 0, 0));
    let mut map = HashMap::new();
    let mut it = 0;
    while unmapped_points.len() > 0 && it < 999_999 {
        it += 1;

        let ((x, y), d, score) = unmapped_points.pop_front().unwrap();

        for (i, (dx, dy)) in DIRECTIONS.iter().enumerate() {
            if x + dx >= 0
                && x + dx < width
                && y + dy >= 0
                && y + dy < height
                && grid[(y + dy) as usize][(x + dx) as usize] != '#'
            {
                let v = map.get(&(x + dx, y + dy));
                let new_score = score + 1 + score_cost_turning(i, d);
                let mut new_v = vec![(i, new_score)];
                if v.is_some() {
                    let o: &Vec<(usize, i32)> = v.unwrap();
                    if o.iter().any(|&(_d, s)| {
                        s + score_cost_turning(d, _d) < new_score || (d == _d && s <= new_score)
                    }) {
                        continue;
                    }
                    let o2 = o
                        .iter()
                        .filter(|&(_d, s)| *s <= new_score + score_cost_turning(*_d, d))
                        .map(|&s| s)
                        .collect::<Vec<(usize, i32)>>();
                    new_v.extend(o2);
                }
                map.insert((x + dx, y + dy), new_v);
                unmapped_points.push_back(((x + dx, y + dy), i, new_score));
            }
        }
    }
    if it == 999_999 {
        println!("Overflowed");
    }
    map
}

fn part_one(file: &String) -> i64 {
    let grid = parse_input(file);
    let (_start, end) = find_start_and_end(&grid);
    let map = generate_map(grid);
    let mut min_end_score = i64::MAX;
    let v = map.get(&(end.0 as i32, end.1 as i32));
    for (_d, score) in v.unwrap() {
        min_end_score = std::cmp::min(*score as i64, min_end_score);
    }
    min_end_score
}

fn part_two(file: &String) -> i64 {
    let grid = parse_input(file);
    let (_start, end) = find_start_and_end(&grid);
    let map = generate_map(grid);

    let mut best_path_points = HashSet::new();
    let mut path_points = VecDeque::new();
    let score = map
        .get(&(end.0 as i32, end.1 as i32))
        .unwrap()
        .iter()
        .fold(i32::MAX, |acc, el| std::cmp::min(el.1, acc));
    path_points.push_back(((end.0 as i32, end.1 as i32), score));
    while path_points.len() > 0 {
        let (point, score) = path_points.pop_front().unwrap();
        best_path_points.insert(point);
        match map.get(&point) {
            None => (),
            Some(v) => {
                for (d, s) in v {
                    if *s <= score {
                        path_points.push_back((
                            (point.0 - DIRECTIONS[*d].0, point.1 - DIRECTIONS[*d].1),
                            *s,
                        ));
                    }
                }
            }
        }
    }
    best_path_points.len() as i64
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
        assert_eq!(result, 45);
    }

    #[test]
    fn test_part_two_second_example() {
        let result = part_two(&String::from(SECOND_EXAMPLE));
        assert_eq!(result, 64);
    }
}

fn main() {
    let file = read_today_data_file(String::from("16"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
