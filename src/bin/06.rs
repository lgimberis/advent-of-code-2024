use advent_of_code_2024::read_today_data_file;

fn parse_data(data: &str) -> Vec<Vec<char>> {
    let mut matrix = Vec::new();
    for line in data.split("\n") {
        let as_bytes = line.chars();
        matrix.push(as_bytes.collect::<Vec<char>>());
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
}

fn main() {
    let data = read_today_data_file(String::from("06"));
    let part_one_result = part_one(&data);
    println!("Part one result: {part_one_result}");
}
