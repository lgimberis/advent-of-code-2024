use advent_of_code_2024::read_today_data_file;
use std::collections::HashSet;

fn parse_input(file: &String) -> Vec<Vec<u32>> {
    file.split("\n")
        .filter(|x| x.len() > 0)
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Should be digits in input"))
                .collect()
        })
        .collect()
}

fn find_trailheads_nonunique(map: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let v = map[y][x];
    if v == 9 {
        return 1;
    }
    let mut from_adjacents = 0;
    if x > 0 && map[y][x - 1] == v + 1 {
        from_adjacents += find_trailheads_nonunique(map, x - 1, y);
    }
    if x < map[0].len() - 1 && map[y][x + 1] == v + 1 {
        from_adjacents += find_trailheads_nonunique(map, x + 1, y);
    }
    if y > 0 && map[y - 1][x] == v + 1 {
        from_adjacents += find_trailheads_nonunique(map, x, y - 1);
    }
    if y < map.len() - 1 && map[y + 1][x] == v + 1 {
        from_adjacents += find_trailheads_nonunique(map, x, y + 1);
    }
    from_adjacents
}

fn find_trailheads(map: &Vec<Vec<u32>>, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let v = map[y][x];
    if v == 9 {
        return HashSet::from([(x, y); 1]);
    }
    let mut from_adjacents = HashSet::new();
    if x > 0 && map[y][x - 1] == v + 1 {
        from_adjacents.extend(find_trailheads(map, x - 1, y));
    }
    if x < map[0].len() - 1 && map[y][x + 1] == v + 1 {
        from_adjacents.extend(find_trailheads(map, x + 1, y));
    }
    if y > 0 && map[y - 1][x] == v + 1 {
        from_adjacents.extend(find_trailheads(map, x, y - 1));
    }
    if y < map.len() - 1 && map[y + 1][x] == v + 1 {
        from_adjacents.extend(find_trailheads(map, x, y + 1));
    }
    from_adjacents
}

fn part_one(file: &String) -> i64 {
    let digit_map = parse_input(file);

    let mut sum_score = 0;
    // Start with naive "dumb" implementation
    for (y, row) in digit_map.iter().enumerate() {
        for (x, &d) in row.iter().enumerate() {
            if d == 0 {
                sum_score += find_trailheads(&digit_map, x, y).len();
            }
        }
    }
    sum_score as i64
}

fn part_two(file: &String) -> i64 {
    let digit_map = parse_input(file);

    let mut sum_score = 0;
    // Start with naive "dumb" implementation
    for (y, row) in digit_map.iter().enumerate() {
        for (x, &d) in row.iter().enumerate() {
            if d == 0 {
                sum_score += find_trailheads_nonunique(&digit_map, x, y);
            }
        }
    }
    sum_score as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&String::from("99999999999\n90123456789\n99999999999"));
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 36);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 81);
    }
}

fn main() {
    let file = read_today_data_file(String::from("10"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
