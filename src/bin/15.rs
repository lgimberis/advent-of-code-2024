use advent_of_code_2024::read_today_data_file;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn get_direction_vector(direction: Direction) -> (isize, isize) {
    match direction {
        Direction::Left => (-1, 0),
        Direction::Right => (1, 0),
        Direction::Up => (0, -1),
        Direction::Down => (0, 1),
    }
}

fn parse_input(file: &String) -> (Vec<Vec<String>>, Vec<Direction>) {
    let mut end_of_warehouse = false;
    let mut instructions = Vec::new();
    let mut rows = Vec::new();
    for line in file.split("\n") {
        if line.len() == 0 {
            end_of_warehouse = true;
            continue;
        }
        if !end_of_warehouse {
            rows.push(line.trim());
        } else {
            for c in line.trim().chars() {
                let dir: Direction = match c {
                    '<' => Direction::Left,
                    '^' => Direction::Up,
                    '>' => Direction::Right,
                    'v' => Direction::Down,
                    _ => panic!("Bad direction"),
                };
                instructions.push(dir);
            }
        }
    }
    let width = rows[0].len();
    let height = rows.len();
    let mut grid = vec![vec!(String::from("."); width); height];
    for (y, row) in rows.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[y][x] = c.to_string();
        }
    }
    (grid, instructions)
}

fn get_move(robot_pos: (usize, usize), direction: Direction) -> (usize, usize) {
    let v = get_direction_vector(direction);
    let new_y = (robot_pos.1 as isize + v.1) as usize;
    let new_x = (robot_pos.0 as isize + v.0) as usize;
    (new_x, new_y)
}
fn test_move(grid: &Vec<Vec<String>>, robot_pos: (usize, usize), direction: Direction) -> &str {
    let (new_x, new_y) = get_move(robot_pos, direction);
    grid[new_y][new_x].as_str()
}

fn push_box(
    grid: &Vec<Vec<String>>,
    box_pos: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    // Try to push the box at this position in this direction.
    // Returns whether the box was pushed and the position of the box / end box in the row

    let mut crawler = (box_pos.0, box_pos.1);
    while grid[crawler.1][crawler.0] == String::from("O") {
        crawler = get_move(crawler, direction);
    }
    if grid[crawler.1][crawler.0] == String::from("#") {
        return None;
    }
    return Some(crawler);
}

fn part_one(file: &String) -> i64 {
    let (mut grid, instructions) = parse_input(file);
    let mut robot_pos = (0usize, 0usize);
    'find_start: for (y, row) in grid.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if s == "@" {
                robot_pos = (x, y);
                // Don't need to know where robot is -> just hide it
                break 'find_start;
            }
        }
    }

    for direction in instructions {
        // Fortunately walls always block robot escape
        match test_move(&grid, robot_pos, direction) {
            "#" => (), // Do nothing
            "." => {
                // Just move there
                grid[robot_pos.1][robot_pos.0] = String::from(".");
                robot_pos = get_move(robot_pos, direction);
                grid[robot_pos.1][robot_pos.0] = String::from("@"); // Don't actually need to know where
                                                                    // robot is?
            }
            "O" => {
                let box_pos = get_move(robot_pos, direction);
                let pushed = push_box(&grid, box_pos, direction);
                if pushed.is_some() {
                    let pushed_pos = pushed.unwrap();
                    grid[box_pos.1][box_pos.0] = String::from("@");
                    grid[robot_pos.1][robot_pos.0] = String::from(".");
                    robot_pos = box_pos;
                    grid[pushed_pos.1][pushed_pos.0] = String::from("O");
                }
            }
            _ => panic!("Unrecognised character"),
        }
        //println!(
        //    "{}",
        //    grid.iter()
        //        .map(|line| line.join(""))
        //        .collect::<Vec<String>>()
        //        .join("\n")
        //);
    }

    let mut sum_of_gps = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if *s == String::from("O") {
                sum_of_gps += x + 100 * y;
            }
        }
    }
    sum_of_gps as i64
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const SMALL_EXAMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn test_part_one_as_given_large() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 10092);
    }

    #[test]
    fn test_part_one_as_given_small() {
        let result = part_one(&String::from(SMALL_EXAMPLE));
        assert_eq!(result, 2028);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("15"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
