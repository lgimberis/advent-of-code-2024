use std::collections::VecDeque;

use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<(usize, usize)> {
    file.split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            let mut l = line.split(",").map(|d| d.parse::<usize>().unwrap());
            (l.next().unwrap(), l.next().unwrap())
        })
        .collect::<Vec<(usize, usize)>>()
}

fn fill_grid(falling_bytes: &[(usize, usize)], width: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec!(false; width); width];

    for &(x, y) in falling_bytes {
        grid[y][x] = true;
    }
    grid
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn find_shortest_path(grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let width = grid.len();
    let mut step_queue: VecDeque<(usize, usize, Vec<(usize, usize)>)> =
        VecDeque::from([(0usize, 0usize, vec![(0usize, 0usize)])]);
    let mut shortest_path_lengths = vec![vec!(usize::MAX; width); width];
    let mut shortest_path = Vec::new();
    //let mut shortest_paths = vec![vec!(Vec::new(); width); width];
    while step_queue.len() > 0 {
        let (x, y, path) = step_queue.pop_front().unwrap();

        if shortest_path_lengths[y][x] <= path.len() {
            continue;
        }
        shortest_path_lengths[y][x] = path.len();
        if y == width - 1 && x == width - 1 {
            shortest_path = path.clone();
        }
        for &(dx, dy) in &DIRECTIONS {
            let new_x = x.checked_add_signed(dx);
            let new_y = y.checked_add_signed(dy);
            if new_x.is_none() || new_y.is_none() {
                continue;
            }
            let new_x = new_x.unwrap();
            let new_y = new_y.unwrap();
            if new_x >= width || new_y >= width || grid[new_y][new_x] {
                continue;
            }
            let mut prospective_path = path.clone();
            prospective_path.push((new_x, new_y));
            step_queue.push_back((new_x, new_y, prospective_path));
        }
    }
    shortest_path
}

fn part_one(file: &String, width: usize, bytes_fallen: usize) -> u64 {
    let falling_bytes = parse_input(file);
    let grid = fill_grid(&falling_bytes[..bytes_fallen], width);
    let path = find_shortest_path(&grid);
    path.len() as u64 - 1
}

fn part_two(file: &String, width: usize, bytes_fallen: usize) -> String {
    let falling_bytes = parse_input(file);
    let mut grid = fill_grid(&falling_bytes[..bytes_fallen], width);
    let mut path = find_shortest_path(&grid);
    for &(x, y) in &falling_bytes[bytes_fallen..] {
        grid[y][x] = true;
        if path.contains(&(x, y)) {
            path = find_shortest_path(&grid);
            if path.len() == 0 {
                return format!("{x},{y}");
            }
        }
    }
    panic!("Couldn't find a byte that blocks the path!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA), 7, 12);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA), 7, 12);
        assert_eq!(result, "6,1");
    }
}

fn main() {
    let file = read_today_data_file(String::from("18"));
    let part_one_result = part_one(&file, 71, 1024);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file, 71, 1024);
    println!("Part two result: {part_two_result}");
}
