use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> String {
    file.replace("\n", "")
}

fn part_one(file: &String) -> i64 {
    let parsed_input = Vec::from_iter(
        parse_input(file)
            .chars()
            .into_iter()
            .map(|x| x.to_digit(10).unwrap() as i64),
    );
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
    0
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
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, -1);
    }
}

fn main() {
    let file = read_today_data_file(String::from("09"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
