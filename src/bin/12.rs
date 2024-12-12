use std::{collections::HashSet, usize::MAX};

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

fn fill_region_sides(
    plant: char,
    grid: &Vec<Vec<char>>,
    mask: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> HashSet<(usize, usize)> {
    mask.insert((x, y));
    let mut directions = Vec::new();
    let mut region = HashSet::new();
    region.insert((x, y));
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
        if grid[new_y][new_x] == plant && !mask.contains(&(new_x, new_y)) {
            let other_region = fill_region_sides(plant, grid, mask, new_x, new_y);
            region.extend(other_region);
        }
    }
    region
}

fn count_region_sides(region: &HashSet<(usize, usize)>) -> i32 {
    // Basic idea: Go left-to-right, and then top-to-bottom
    // Each time we enter and subsequently leave the shape, we add 2 sides
    // Each time we note two points as being entrances/exits, we mark all points in transverse
    // directions as visited
    let mut left = MAX;
    let mut top = MAX;
    let mut right = 0;
    let mut bottom = 0;
    for &(x, y) in region {
        left = std::cmp::min(left, x);
        right = std::cmp::max(right, x);
        top = std::cmp::min(top, y);
        bottom = std::cmp::max(bottom, y);
    }

    let mut inside_shape = false;
    let mut entrance = 0;
    let mut sides = 0;
    // Left-to-right
    for y in top..=bottom {
        for x in left..=right {
            if region.contains(&(x, y)) {
                if !inside_shape {
                    entrance = x;
                    inside_shape = true;
                }
            } else {
                if inside_shape {
                    sides += 2;
                    if y > 0
                        && region.contains(&(entrance, y - 1))
                        && (entrance == 0 || !region.contains(&(entrance - 1, y - 1)))
                    {
                        sides -= 1; // Left edge entrance already counted
                    }
                    if y > 0 && region.contains(&(x - 1, y - 1)) && !region.contains(&(x, y - 1)) {
                        sides -= 1; // Right edge exit already counted
                    }
                    inside_shape = false;
                }
            }
        }
        if inside_shape {
            sides += 2;
            if y > 0
                && region.contains(&(entrance, y - 1))
                && (entrance == 0 || !region.contains(&(entrance - 1, y - 1)))
            {
                sides -= 1; // Left edge entrance already counted
            }
            if y > 0 && region.contains(&(right, y - 1)) {
                sides -= 1; // Right edge exit already counted
            }
            inside_shape = false;
        }
    }
    // Top-to-bottom
    inside_shape = false;
    for x in left..=right {
        for y in top..=bottom {
            if region.contains(&(x, y)) {
                if !inside_shape {
                    entrance = y;
                    inside_shape = true;
                }
            } else {
                if inside_shape {
                    sides += 2;
                    if x > 0
                        && region.contains(&(x - 1, entrance))
                        && (entrance == 0 || !region.contains(&(x - 1, entrance - 1)))
                    {
                        sides -= 1; // Top edge entrance already counted
                    }
                    if x > 0 && region.contains(&(x - 1, y - 1)) && !region.contains(&(x - 1, y)) {
                        sides -= 1; // Bottom edge exit already coutned
                    }
                    inside_shape = false;
                }
            }
        }
        if inside_shape {
            sides += 2;
            if x > 0
                && region.contains(&(x - 1, entrance))
                && (entrance == 0 || !region.contains(&(x - 1, entrance - 1)))
            {
                sides -= 1; // Top edge entrance already counted
            }
            if x > 0 && region.contains(&(x - 1, bottom)) {
                sides -= 1; // Bottom edge exit already coutned
            }
            inside_shape = false;
        }
    }
    sides
}

fn part_two(file: &String) -> i64 {
    let grid = parse_input(file);
    let mut mask: HashSet<(usize, usize)> = HashSet::new();
    let mut price = 0;

    for (y, row) in grid.iter().enumerate() {
        for (x, &plant) in row.iter().enumerate() {
            if !mask.contains(&(x, y)) {
                let region = fill_region_sides(plant, &grid, &mut mask, x, y);
                price += region.len() as i32 * count_region_sides(&region);
            }
        }
    }
    price as i64
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

    const PART_TWO_BASIC: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";

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
    fn test_part_two_basic() {
        let result = part_two(&String::from(BASIC_EXAMPLE));
        assert_eq!(result, 80);
    }

    #[test]
    fn test_part_two_e() {
        let result = part_two(&String::from(PART_TWO_BASIC));
        assert_eq!(result, 236);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 1206);
    }
}

fn main() {
    let file = read_today_data_file(String::from("12"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
