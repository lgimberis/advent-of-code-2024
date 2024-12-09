use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> Vec<i64> {
    Vec::from_iter(
        file.replace("\n", "")
            .chars()
            .into_iter()
            .map(|x| x.to_digit(10).unwrap() as i64),
    )
}

fn part_one(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut input_left = 0;
    let mut input_right = parsed_input.len() - 1;
    let mut right_leftovers = parsed_input[input_right];
    let mut checksum = 0;
    let mut output_index = 0;
    let mut this_block_is_file = true;
    while input_left < input_right {
        if this_block_is_file {
            // File space; include as normal
            let n = parsed_input[input_left];
            checksum += (input_left as i64 / 2) * (n * output_index + n * (n - 1) / 2); // n-1 due to starting at zero
            output_index += n;
        } else {
            // Empty space; fill from right
            let mut empty_spaces = parsed_input[input_left];
            while empty_spaces > 0 && input_left < input_right {
                if right_leftovers == 0 {
                    input_right -= 2;
                    right_leftovers = parsed_input[input_right];
                    continue;
                }
                right_leftovers -= 1;
                checksum += output_index * input_right as i64 / 2;
                empty_spaces -= 1;
                output_index += 1;
            }
            if right_leftovers == 0 {
                input_right -= 2;
                right_leftovers = parsed_input[input_right];
            }
        }
        input_left += 1;
        this_block_is_file = !this_block_is_file;
    }
    for _i in 0..right_leftovers {
        checksum += output_index * input_right as i64 / 2;
        output_index += 1;
    }
    checksum
}

fn part_two(file: &String) -> i64 {
    let parsed_input = parse_input(file);
    let mut holes = Vec::new();
    let mut files = Vec::new();
    let mut moved_files = Vec::new();
    let mut index: usize = 0;
    for (i, &x) in parsed_input.iter().enumerate() {
        if i % 2 == 0 {
            files.push((index, x));
        } else {
            holes.push((index, x));
        }
        index += x as usize;
    }
    files.reverse();
    'files: for (_index, &(i, x)) in files.iter().enumerate() {
        // Try to move to earliest hole
        for (hole_index, &(j, y)) in holes.iter().enumerate() {
            if y >= x && j < i {
                moved_files.push((j, x));
                holes[hole_index] = (j + x as usize, y - x);
                continue 'files;
            }
        }
        // Nowhere to move to
        moved_files.push((i, x));
    }
    let mut checksum = 0;
    let len = moved_files.len();
    for (index, &(i, x)) in moved_files.iter().enumerate() {
        checksum += (len - 1 - index) as i64 * (x * i as i64 + x * (x - 1) / 2);
    }
    checksum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "2333133121414131402";

    #[test]
    fn test_part_one_simple() {
        let result = part_one(&String::from("11111"));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_part_two_simple() {
        let result = part_two(&String::from("11111"));
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 2858);
    }
}

fn main() {
    let file = read_today_data_file(String::from("09"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
