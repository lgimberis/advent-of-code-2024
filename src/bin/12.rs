use std::collections::HashSet;

use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<Vec<char>> {
    file.split("\n")
        .filter(|x| x.len() > 0)
        .map(|x| x.chars().into_iter().collect())
        .collect()
}

fn fill_region(
    plant: char,
    grid: &Vec<Vec<char>>,
    mask: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> (i32, i32) {
    mask.insert((x, y));
    let mut perimeter = 4;
    let mut area = 1;
    let mut directions = Vec::new();
    if x > 0 {
        directions.push((x - 1, y));
    }
    if x < grid[y].len() - 1 {
        directions.push((x + 1, y));
    }
    if y > 0 {
        directions.push((x, y - 1));
    }
    if y < grid.len() - 1 {
        directions.push((x, y + 1));
    }
    for (new_x, new_y) in directions {
        perimeter -= 1;
        if grid[new_y][new_x] != plant {
            perimeter += 1;
        } else if !mask.contains(&(new_x, new_y)) {
            let (other_area, other_perimeter) = fill_region(plant, grid, mask, new_x, new_y);
            area += other_area;
            perimeter += other_perimeter;
        }
    }
    (area, perimeter)
}

fn part_one(file: &String) -> i64 {
    let grid = parse_input(file);
    let mut mask: HashSet<(usize, usize)> = HashSet::new();
    let mut price = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &plant) in row.iter().enumerate() {
            if !mask.contains(&(x, y)) {
                let (area, perimeter) = fill_region(plant, &grid, &mut mask, x, y);
                price += area * perimeter;
            }
        }
    }
    price as i64
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_EXAMPLE: &str = "AAAA
BBCD
BBCC
EEEC";

    const OTHER_EXAMPLE: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

    const EXAMPLE_DATA: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_part_one_basic() {
        let result = part_one(&String::from(BASIC_EXAMPLE));
        assert_eq!(result, 140);
    }

    #[test]
    fn test_part_one_other() {
        let result = part_one(&String::from(OTHER_EXAMPLE));
        assert_eq!(result, 772);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("12"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
