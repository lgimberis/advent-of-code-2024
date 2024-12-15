use advent_of_code_2024::read_today_data_file;

#[derive(Clone, Copy, Debug, PartialEq)]
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

fn parse_input(file: &String) -> (Vec<&str>, Vec<Direction>) {
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
    (rows, instructions)
}

fn interpret_single_warehouse(warehouse: Vec<&str>) -> Vec<Vec<String>> {
    let width = warehouse[0].len();
    let height = warehouse.len();
    let mut grid = vec![vec!(String::from("."); width); height];
    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid[y][x] = c.to_string();
        }
    }
    grid
}

fn interpret_double_warehouse(warehouse: Vec<&str>) -> Vec<Vec<String>> {
    let width = 2 * warehouse[0].len();
    let height = warehouse.len();
    let mut grid = vec![vec!(String::from("."); width); height];
    for (y, row) in warehouse.iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let left = match c {
                'O' => '[',
                _c => _c,
            };
            let right = match c {
                'O' => ']',
                '@' => '.',
                _c => _c,
            };
            grid[y][2 * x] = left.to_string();
            grid[y][2 * x + 1] = right.to_string();
        }
    }
    grid
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

fn push_box_vertical(
    grid: &Vec<Vec<String>>,
    starting_pos: (usize, usize),
    direction: Direction,
) -> Option<Vec<(usize, usize)>> {
    let c = &grid[starting_pos.1][starting_pos.0];
    if c != "[" && c != "]" {
        // This isn't a box
        return Some(Vec::new());
    }
    let v = get_direction_vector(direction);
    let other_box_half_offset: isize = if c == "[" { 1 } else { -1 };
    let left_pos = if c == "[" {
        starting_pos
    } else {
        (starting_pos.0 - 1, starting_pos.1)
    };
    let above_pos = (starting_pos.0, (starting_pos.1 as isize + v.1) as usize);
    let above = &grid[above_pos.1][above_pos.0];
    let other_pos = (
        (above_pos.0 as isize + other_box_half_offset) as usize,
        above_pos.1,
    );
    let other = &grid[other_pos.1][other_pos.0];
    if other == "#" || above == "#" {
        return None;
    }
    if above == c {
        return match push_box_vertical(grid, (left_pos.0, above_pos.1), direction) {
            None => None,
            Some(mut x) => {
                x.push(left_pos);
                return Some(x);
            }
        };
    }
    let mut boxes = Vec::new();
    if above == "[" || above == "]" {
        match push_box_vertical(grid, above_pos, direction) {
            None => {
                return None;
            }
            Some(x) => boxes.extend(x),
        }
    }
    if other == "[" || other == "]" {
        match push_box_vertical(grid, other_pos, direction) {
            None => {
                return None;
            }
            Some(x) => boxes.extend(x),
        }
    }
    boxes.push(left_pos);
    Some(boxes)
}

fn push_box(
    grid: &Vec<Vec<String>>,
    box_pos: (usize, usize),
    direction: Direction,
) -> Option<Vec<(usize, usize)>> {
    // Try to push the box at this position in this direction.
    // Returns whether the box was pushed and the position of the box / end box in the row

    let boxes = vec![String::from("O"), String::from("["), String::from("]")];
    if grid[box_pos.1][box_pos.0] == "O"
        || direction == Direction::Left
        || direction == Direction::Right
    {
        let mut crawler = (box_pos.0, box_pos.1);
        while boxes.contains(&grid[crawler.1][crawler.0]) {
            crawler = get_move(crawler, direction);
        }
        if grid[crawler.1][crawler.0] == String::from("#") {
            return None;
        }
        if grid[box_pos.1][box_pos.0] == "O" {
            return Some(vec![crawler]);
        } else {
            let n = (crawler.0.abs_diff(box_pos.0) / 2) as isize;
            let mut b = Vec::new();
            let v = get_direction_vector(direction);
            let offset = if v.0 > 0 { 0 } else { -1 };
            for x in 1..=n {
                b.push((
                    (crawler.0 as isize - 2 * v.0 * x + offset) as usize,
                    crawler.1,
                ));
            }
            return Some(b);
        }
    }
    push_box_vertical(grid, box_pos, direction)
}

fn get_gps(grid: &Vec<Vec<String>>) -> i64 {
    let mut sum_of_gps = 0;
    for (y, row) in grid.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if *s == String::from("O") || *s == String::from("[") {
                sum_of_gps += x + 100 * y;
            }
        }
    }
    sum_of_gps as i64
}

fn find_robot(grid: &Vec<Vec<String>>) -> (usize, usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, s) in row.iter().enumerate() {
            if s == "@" {
                return (x, y);
            }
        }
    }
    panic!("Couldn't find robot!");
}

fn part_one(file: &String) -> i64 {
    let (warehouse, instructions) = parse_input(file);
    let mut grid = interpret_single_warehouse(warehouse);
    let mut robot_pos = find_robot(&grid);

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
                    let pushed_pos = pushed.unwrap()[0];
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

    get_gps(&grid)
}

fn part_two(file: &String) -> i64 {
    let (warehouse, instructions) = parse_input(file);
    let mut grid = interpret_double_warehouse(warehouse);
    let mut robot_pos = find_robot(&grid);

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
            "[" | "]" => {
                let box_pos = get_move(robot_pos, direction);
                let pushed = push_box(&grid, box_pos, direction);
                if pushed.is_some() {
                    let boxes = pushed.unwrap();
                    for block in &boxes {
                        // Remove previous blockes
                        grid[block.1][block.0] = String::from(".");
                        grid[block.1][block.0 + 1] = String::from(".");
                    }
                    for block in &boxes {
                        // Move up
                        let moved = get_move(*block, direction);
                        grid[moved.1][moved.0] = String::from("[");
                        grid[moved.1][moved.0 + 1] = String::from("]");
                    }
                    grid[box_pos.1][box_pos.0] = String::from("@");
                    grid[robot_pos.1][robot_pos.0] = String::from(".");
                    robot_pos = box_pos;
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

    get_gps(&grid)
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
        assert_eq!(result, 9021);
    }
}

fn main() {
    let file = read_today_data_file(String::from("15"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
