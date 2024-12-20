use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<char>> {
    file.lines().map(|line| line.chars().collect()).collect()
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_racetrack(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let height = grid.len();
    let width = grid[0].len();

    let mut position = (usize::MAX, usize::MAX);
    'outer: for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                position = (x, y);
                break 'outer;
            }
        }
    }

    if position == (usize::MAX, usize::MAX) {
        panic!("Start point not found");
    }

    let mut previous_direction = 0;
    let mut racetrack = vec![position];
    while grid[position.1][position.0] != 'E' {
        for (i, direction) in DIRECTIONS.iter().enumerate() {
            if racetrack.len() == 1 || i != (previous_direction + 2) % 4 {
                let new_x = position.0.checked_add_signed(direction.0);
                let new_y = position.1.checked_add_signed(direction.1);
                if new_x.is_none() || new_y.is_none() {
                    continue;
                }
                let new_x = new_x.unwrap();
                let new_y = new_y.unwrap();
                if new_x >= width || new_y >= height || grid[new_y][new_x] == '#' {
                    continue;
                }
                previous_direction = i;
                racetrack.push((new_x, new_y));
                position = (new_x, new_y);
                break;
            }
        }
    }
    racetrack
}

fn find_cheats_of_at_least(n: usize, racetrack: &Vec<(usize, usize)>) -> u64 {
    // Returns the number of cheats that save `n` picoseconds or more
    let mut count = 0;
    for i in n..racetrack.len() {
        let cheat_end = racetrack[i];
        for j in 0..(i - n) {
            let cheat_start = racetrack[j];
            if cheat_end.0.abs_diff(cheat_start.0) + cheat_end.1.abs_diff(cheat_start.1) <= 2 {
                count += 1;
            }
        }
    }
    count
}

fn part_one(file: &String, n: usize) -> u64 {
    let grid = parse_input(file);
    let racetrack = find_racetrack(&grid);
    find_cheats_of_at_least(n, &racetrack)
}

fn part_two(file: &String) -> u64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA), 20);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, u64::MAX);
    }
}

fn main() {
    let file = read_today_data_file(String::from("20"));
    let part_one_result = part_one(&file, 100);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
