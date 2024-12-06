use advent_of_code_2024::read_today_data_file;
use std::collections::HashSet;

fn parse_data(data: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in data.split("\n") {
        let as_bytes = line.chars();
        let len = as_bytes.clone().count();
        if len > 0 {
            matrix.push(as_bytes.collect::<Vec<char>>());
        }
    }
    matrix
}

fn part_one(data: &str) -> u32 {
    let matrix = parse_data(data);
    let mut mask = matrix
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();

    let height = matrix.len();
    let width = matrix[0].len();
    // Find start position
    let mut x = width as i32;
    let mut y = height as i32;
    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                y = i as i32;
                x = j as i32;
                break 'outer;
            }
        }
    }
    mask[y as usize][x as usize] = true;
    let mut it = 0;
    let mut turns = 0;
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 && it < 999_999_999 {
        match matrix[y as usize][x as usize] {
            '#' => {
                // Undo movement and rotate
                x = x - DIRECTIONS[turns % 4].0;
                y = y - DIRECTIONS[turns % 4].1;
                turns += 1;
            }
            _ => {
                mask[y as usize][x as usize] = true;
                x = x + DIRECTIONS[turns % 4].0;
                y = y + DIRECTIONS[turns % 4].1;
            }
        }

        it += 1;
    }

    let mut distinct_positions = 0;
    for row in mask {
        for b in row {
            if b {
                distinct_positions += 1;
            }
        }
    }
    distinct_positions
}

fn part_two(data: &str) -> u32 {
    let matrix = parse_data(data);

    // General algorithm:
    // Try to put an obstacle directly in front of the guard
    // If that creates an infinite loop, set the mask for that obstacle
    // (so that we don't double count)
    let mut mask = matrix
        .iter()
        .map(|row| row.iter().map(|_| false).collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    // Otherwise, step forward and apply any rules

    let height = matrix.len();
    let width = matrix[0].len();
    // Find start position
    let mut x = width as i32;
    let mut y = height as i32;
    'outer: for (i, row) in matrix.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == '^' {
                y = i as i32;
                x = j as i32;
                break 'outer;
            }
        }
    }
    let start_x = x;
    let start_y = y;
    let mut it = 0;
    let mut turns = 0;
    let mut path = HashSet::new();
    const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
    while x >= 0 && x < width as i32 && y >= 0 && y < height as i32 && it < 999_999_999 {
        match matrix[y as usize][x as usize] {
            '#' => {
                // Undo movement and rotate
                x = x - DIRECTIONS[turns % 4].0;
                y = y - DIRECTIONS[turns % 4].1;
                turns += 1;
            }
            _ => {
                // See what happens if we put an obstacle ahead
                let obstacle_x = x + DIRECTIONS[turns % 4].0;
                let obstacle_y = y + DIRECTIONS[turns % 4].1;
                let mut test_x = x;
                let mut test_y = y;
                let mut test_turns = turns + 1;
                let mut test_it = 0;
                let mut obstacles_visited: HashSet<(i32, i32, usize)> = HashSet::new();
                obstacles_visited.insert((obstacle_x, obstacle_y, turns % 4));
                if obstacle_x >= 0
                    && obstacle_x < width as i32
                    && obstacle_y >= 0
                    && obstacle_y < height as i32
                    && matrix[obstacle_y as usize][obstacle_x as usize] != '#'
                    && !path.contains(&(obstacle_x, obstacle_y))
                {
                    while test_x >= 0
                        && test_x < width as i32
                        && test_y >= 0
                        && test_y < height as i32
                        && test_it < 999_999
                    {
                        if matrix[test_y as usize][test_x as usize] == '#'
                            || (obstacle_x == test_x && obstacle_y == test_y)
                        {
                            if obstacles_visited.contains(&(test_x, test_y, test_turns % 4)) {
                                mask[obstacle_y as usize][obstacle_x as usize] = true;
                                break;
                            }
                            // Undo movement and rotate
                            obstacles_visited.insert((test_x, test_y, test_turns % 4));
                            test_x -= DIRECTIONS[test_turns % 4].0;
                            test_y -= DIRECTIONS[test_turns % 4].1;
                            test_turns += 1;
                        } else {
                            test_x += DIRECTIONS[test_turns % 4].0;
                            test_y += DIRECTIONS[test_turns % 4].1;
                        }

                        test_it += 1;
                    }
                }

                // Move forward
                x = x + DIRECTIONS[turns % 4].0;
                y = y + DIRECTIONS[turns % 4].1;
                path.insert((x, y));
            }
        }

        it += 1;
    }

    let mut loop_obstacles = 0;
    for (y, row) in mask.iter().enumerate() {
        for (x, &b) in row.iter().enumerate() {
            if b && (x != start_x as usize || y != start_y as usize) {
                loop_obstacles += 1;
            }
        }
    }

    loop_obstacles
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_as_given() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = part_one(data);
        assert_eq!(result, 41);
    }

    #[test]
    fn test_part_two_as_given() {
        let data = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let result = part_two(data);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part_two_not_covered() {
        // In which I attempt to reconcile the fact that tests run but I have the wrong answer by
        // trying to find a set of input data that is wrong
        let data = "
....#.....
......#...
....^.....
.....#....";
        let result = part_two(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_not_covered2() {
        let data = "
..###.....
......#...
#...^.....
.....#....";
        let result = part_two(data);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two_phantom_loop() {
        let data = "
.....
...#.
#^...
..#..";
        let result = part_two(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_phantom_dangling_loop() {
        let data = "
...#..
.....#
.^#...
....#.";
        let result = part_two(data);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_two_not_blocks_approach() {
        let data = "
.#.
#.#
...
...
.^.
";
        let result = part_two(data);
        assert_eq!(result, 0);
    }
}

fn main() {
    let data = read_today_data_file(String::from("06"));
    let part_one_result = part_one(&data);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&data);
    println!("Part two result: {part_two_result}");
}
