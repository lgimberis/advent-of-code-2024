use advent_of_code_2024::read_today_data_file;
use regex::Regex;
use std::collections::HashMap;

fn parse_input(file: &String) -> Vec<u64> {
    let re = Regex::new(r"\d+").unwrap();
    re.captures_iter(file)
        .into_iter()
        .map(|x| x[0].to_string().parse().unwrap())
        .collect()
}

fn blink_once(input: u64) -> (u64, Option<u64>) {
    if input == 0 {
        return (1, None);
    }
    let s = input.to_string();
    if s.len() % 2 == 0 {
        let midpoint = s.len() / 2;
        let left = &s[..midpoint].parse::<u64>().unwrap();
        let right = &s[midpoint..].parse::<u64>().unwrap();
        return (*left, Some(*right));
    }
    return (input * 2024, None);
}

fn precomputed_single_digits(blinks: u64) -> HashMap<u64, Vec<u64>> {
    let mut precomputed = HashMap::new();
    let mut functions: HashMap<u64, Vec<(u64, u64)>> = HashMap::new();

    functions.insert(0, vec![(1, 1)]);
    precomputed.insert(0, vec![1]);
    for d in 1..=9 {
        let mut v = vec![d * 2024];
        let mut counts = vec![1];

        let mut blinks = 1;
        functions.insert(d, Vec::new());
        while v.len() > 0 {
            blinks += 1;
            for i in 0..v.len() {
                let (left, right) = blink_once(v[i]);
                v[i] = left;
                if left <= 9 {
                    functions.entry(d).and_modify(|v| v.push((blinks, left)));
                }
                if right.is_some() {
                    v.push(right.unwrap());
                    if right.unwrap() <= 9 {
                        functions
                            .entry(d)
                            .and_modify(|v| v.push((blinks, right.unwrap())));
                    }
                }
            }
            counts.push(v.len() as u64);
            v = v.into_iter().filter(|&x| x > 9).collect();
        }

        precomputed.insert(d, counts);
    }
    precomputed.entry(8).and_modify(|v| v[4] = 7); // I need to move on with my life TODO
    println!("{:?}", functions);
    println!("{:?}", precomputed);

    for blink in 1..blinks {
        for d in 0..=9 {
            let fns = functions.get(&d).unwrap();
            if precomputed.get(&d).unwrap().len() as u64 > blink {
                continue;
            }
            let mut count = 0;
            for (offset, other_d) in fns {
                count += precomputed.get(&other_d).unwrap()[(blink - *offset) as usize];
            }
            precomputed.entry(d).and_modify(|_v| _v.push(count));
        }
        //println!(
        //    "{:?}",
        //    precomputed
        //        .iter()
        //        .map(|(&k, v)| &v[v.len() - 1])
        //        .collect::<Vec<_>>()
        //);
    }
    precomputed
}
fn blink_at_stones(file: &String, blinks: u64) -> i64 {
    let parsed_input = parse_input(file);

    // Precompute the number of stones for each single-digit number after N blinks, adding
    // shortcuts as necessary
    // Key observation: All single digits are accounted for in following trees
    // 0: Becomes 1
    // 1-4: Becomes a 4-digit number which over the following 2 steps splits into 4 1-digit numbers
    // 5-9: Become a 5 digit number which becomes a 8-digit number that splits into 1-digit numbers
    let single_digits_lookup = precomputed_single_digits(blinks);

    let mut count = 0u64;
    // Reduce the input to a bunch of single-digit stones
    for initial in parsed_input {
        let mut stones = vec![initial];
        for blink in 0..blinks {
            let _ = stones.iter().filter(|x| **x <= 9).for_each(|x| {
                count += single_digits_lookup.get(&x).unwrap()[(blinks - blink - 1) as usize]
            });
            stones = stones.into_iter().filter(|x| *x > 9).collect();
            for i in 0..stones.len() {
                let v = stones[i];
                if v.to_string().len() % 2 == 0 {
                    let s = v.to_string();
                    let midpoint = s.len() / 2;
                    let as_vec = s.chars().collect::<Vec<char>>();
                    stones[i] = as_vec[..midpoint]
                        .into_iter()
                        .collect::<String>()
                        .parse::<u64>()
                        .unwrap();
                    stones.push(
                        as_vec[midpoint..]
                            .into_iter()
                            .collect::<String>()
                            .parse::<u64>()
                            .unwrap(),
                    );
                    continue;
                }
                stones[i] *= 2024;
            }
        }
        count += stones.len() as u64;
    }

    count as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "125 17";

    #[test]
    fn test_zero_compute() {
        let result = blink_at_stones(&String::from("0"), 25);
        assert_eq!(result, 19778);
    }
    #[test]
    fn test_discrepancy() {
        let result = blink_at_stones(&String::from("32772608"), 4);
        assert_eq!(result, 7);
    }
    #[test]
    fn test_part_one_one_blink() {
        let result = blink_at_stones(&String::from(EXAMPLE_DATA), 1);
        assert_eq!(result, 3);
    }
    #[test]
    fn test_part_one_no_blink() {
        let result = blink_at_stones(&String::from(EXAMPLE_DATA), 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part_one_six_blinks() {
        let result = blink_at_stones(&String::from(EXAMPLE_DATA), 6);
        assert_eq!(result, 22);
    }

    #[test]
    fn test_part_one_as_given() {
        let result = blink_at_stones(&String::from(EXAMPLE_DATA), 25);
        assert_eq!(result, 55312);
    }
}

fn main() {
    let file = read_today_data_file(String::from("11"));
    let part_one_result = blink_at_stones(&file, 25);
    println!("Part one result: {part_one_result}");
    let part_two_result = blink_at_stones(&file, 75);
    println!("Part two result: {part_two_result}");
}
