use itertools::Itertools;
use std::collections::{HashMap, HashSet};

use advent_of_code_2024::read_today_data_file;

fn parse_input(file: &String) -> HashMap<&str, HashSet<&str>> {
    let connections = file
        .lines()
        .filter(|s| !s.is_empty())
        .map(|line| line.split("-").collect_vec());
    let mut map = HashMap::new();
    for connection in connections {
        match connection[..] {
            [left, right, ..] => {
                //let left = connection.next().unwrap();
                //let right = connection.next().unwrap();
                let l = map.entry(left).or_insert(HashSet::new());
                l.insert(right);
                let r = map.entry(right).or_insert(HashSet::new());
                r.insert(left);
            }
            _ => {}
        }
    }
    map
}

fn part_one(file: &String) -> u64 {
    let connection_map = parse_input(file);
    let mut combinations = HashSet::new();
    for (source, destinations) in &connection_map {
        if source.chars().next().unwrap() != 't' {
            continue;
        }
        for destination in destinations {
            for possible_mutual in connection_map.get(destination).unwrap() {
                if destinations.contains(possible_mutual) {
                    let mut v = vec![possible_mutual, destination, source];
                    v.sort();
                    combinations.insert(v.iter().join(","));
                }
            }
        }
    }
    combinations.len() as u64
}

fn part_two(file: &String) -> String {
    let connection_map = parse_input(file);
    let mut parties: Vec<HashSet<&str>> = Vec::new();
    for (source, destinations) in &connection_map {
        'find_membership: for party in parties.iter_mut() {
            for computer in party.iter() {
                if !destinations.contains(computer) {
                    continue 'find_membership;
                }
            }
            party.insert(source);
        }
        for destination in destinations {
            parties.push(HashSet::from([*source, *destination]));
        }
    }
    //println!("{:?}", parties);
    let (_longest_length, longest_party) = parties.iter().fold((0, HashSet::new()), |acc, el| {
        if el.len() > acc.0 {
            (el.len(), el.clone())
        } else {
            acc
        }
    });
    longest_party.iter().sorted().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";

    #[test]
    fn test_part_one_as_given() {
        let result = part_one(&String::from(EXAMPLE_DATA));
        assert_eq!(result, 7);
    }

    #[test]
    fn test_part_two_as_given() {
        let result = part_two(&String::from(EXAMPLE_DATA));
        assert_eq!(result, "co,de,ka,ta");
    }
}

fn main() {
    let file = read_today_data_file(String::from("23"));
    let part_one_result = part_one(&file);
    println!("Part one result: {part_one_result}");
    let part_two_result = part_two(&file);
    println!("Part two result: {part_two_result}");
}
