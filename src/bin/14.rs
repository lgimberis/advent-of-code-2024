use core::time;
use std::thread::sleep;

use advent_of_code_2024::read_today_data_file;
use regex::Regex;

fn parse_input(file: &String) -> Vec<(i32, i32, i32, i32)> {
    let lines = file.split("\n").filter(|x| x.len() > 0);
    let re = Regex::new(r"(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let mut out = Vec::new();
    for line in lines {
        let cap = re.captures(line).unwrap();
        out.push((
            cap[1].parse::<i32>().unwrap(),
            cap[2].parse::<i32>().unwrap(),
            cap[3].parse::<i32>().unwrap(),
            cap[4].parse::<i32>().unwrap(),
        ));
    }
    out
}

fn part_one(width: i32, height: i32, file: &String, seconds: i32) -> u64 {
    let robots = parse_input(file);
    let mut quadrants = vec![vec!(0u64; 2); 2];
    for robot in robots {
        let final_x = (robot.0 as i32 + seconds * robot.2).rem_euclid(width);
        let final_y = (robot.1 as i32 + seconds * robot.3).rem_euclid(height);
        if final_x == width / 2 || final_y == height / 2 {
            continue;
        }
        let quadrant_x = if final_x > (width / 2) { 1 } else { 0 };
        let quadrant_y = if final_y > (height / 2) { 1 } else { 0 };
        quadrants[quadrant_x][quadrant_y] += 1;
    }
    quadrants[0][0] * quadrants[0][1] * quadrants[1][0] * quadrants[1][1]
}

fn part_two(width: i32, height: i32, file: &String) -> () {
    let mut robots = parse_input(file);
    let mut quadrants = vec![vec!(0u64; 2); 2];
    let mut it = 0;
    while it < 99_999_999 {
        it += 1;
        quadrants[0][0] = 0;
        quadrants[0][1] = 0;
        quadrants[1][0] = 0;
        quadrants[1][1] = 1;
        let mut centre_robots = 0;
        for robot in &mut robots {
            // Critical assumption: quadrants are symmetric in x IFF the tree is displayed
            robot.0 = (robot.0 + robot.2).rem_euclid(width);
            robot.1 = (robot.1 + robot.3).rem_euclid(height);
            if (robot.0 > (width / 2 - width / 16) && robot.0 < (width / 2 + width / 16))
                && robot.1 > height / 2 - height / 16
                && robot.1 > height / 2 + height / 16
            {
                centre_robots += 1;
            }
            if robot.0 == width / 2 || robot.1 == height / 2 {
                continue;
            }
            let quadrant_x = if robot.0 > (width / 2) { 1 } else { 0 };
            let quadrant_y = if robot.1 > (height / 2) { 1 } else { 0 };
            quadrants[quadrant_x][quadrant_y] += 1;
        }
        if centre_robots > 40 {
            // Assumed criterion: there's something in the middle (as opposed
            // to random ordering)
            println!("{it}: {centre_robots}");
            // Display
            let mut buffer = vec![vec!("."; width as usize); height as usize];
            for robot in &robots {
                buffer[robot.1 as usize][robot.0 as usize] = "X";
            }
            let display = buffer
                .iter()
                .map(|line| line.join(""))
                .collect::<Vec<String>>()
                .join("\n");
            println!("{}", display);
            sleep(time::Duration::from_millis(50));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(11, 7, &String::from(EXAMPLE_DATA), 100);
        assert_eq!(result, 12);
    }
}

fn main() {
    let file = read_today_data_file(String::from("14"));
    let part_one_result = part_one(101, 103, &file, 100);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(101, 103, &file);
}
