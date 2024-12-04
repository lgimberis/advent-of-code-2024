use advent_of_code_2024::read_today_data_file;

fn part_one(data: &String) -> u32 {
    let width = data.find("\n").or(Some(data.len())).unwrap();
    let width = data[0..width].trim().len();
    let data = data.replace(" ", "");
    let chars = data.as_bytes();
    let mut matches: u32 = 0;

    fn test_ahead(data: &[u8], i: usize, step: usize) -> u32 {
        if data.len() <= i + step * 3 {
            return 0;
        }
        let word = String::from_utf8(
            [
                data[i],
                data[i + step],
                data[i + step * 2],
                data[i + step * 3],
            ]
            .to_vec(),
        )
        .unwrap();
        if word == "XMAS" || word == "SAMX" {
            return 1;
        }
        0
    }

    for (i, &letter) in chars.iter().enumerate() {
        if letter == b'S' || letter == b'X' {
            matches += test_ahead(chars, i, 1);
            matches += test_ahead(chars, i, width + 1);
            if width >= 4 {
                matches += test_ahead(chars, i, width + 2) + test_ahead(chars, i, width);
            }
        }
    }
    matches
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_given() {
        let data = "MMMSXXMASM
        MSAMXMSMSA
        AMXSXMAAMM
        MSAMASMSMX
        XMASAMXAMM
        XXAMMXXAMA
        SMSMSASXSS
        SAXAMASAAA
        MAMMMXMMMM
        MXMXAXMASX";

        let result = part_one(&data.to_string());
        assert_eq!(result, 18);
    }

    #[test]
    fn test_part_one_backwards() {
        let data = "SAMX";

        let result = part_one(&data.to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_one_downwards() {
        let data = "X\nM\nA\nS";

        let result = part_one(&data.to_string());
        assert_eq!(result, 1);
    }

    #[test]
    fn test_part_one_diagonals() {
        let data = "XWWS\nWMAW\nWMAW\nXWWS";

        let result = part_one(&data.to_string());
        assert_eq!(result, 2);
    }
}
fn main() {
    let file = read_today_data_file(String::from("04"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
}
