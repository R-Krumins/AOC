use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let (rules, updates) = file_content.split_once("\n\n").unwrap();

    //value: number, key: must be before
    let mut rule_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for rule in rules.lines() {
        let (n1, n2) = rule.split_once("|").unwrap();

        rule_map.entry(n1).or_insert_with(HashSet::new).insert(n2);
    }

    //println!("{:?}", rule_map);

    let mut result: i32 = 0;
    for update in updates.lines() {
        let update: Vec<&str> = update.split(",").collect();
        let mut is_valid_update = true;
        for i in 1..update.len() {
            if let Some(before) = rule_map.get(update[i]) {
                for j in 0..i {
                    if before.contains(update[j]) {
                        is_valid_update = false;
                        break;
                    }
                }
            }

            if !is_valid_update {
                break;
            }
        }

        if is_valid_update {
            let middle: i32 = update[update.len() / 2].parse().unwrap();
            result += middle;
        }
    }

    println!("{}", result);
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let (rules, updates) = file_content.split_once("\n\n").unwrap();

    //value: number, key: must be before
    let mut rule_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for rule in rules.lines() {
        let (n1, n2) = rule.split_once("|").unwrap();

        rule_map.entry(n1).or_insert_with(HashSet::new).insert(n2);
    }

    let mut result: i32 = 0;
    for update in updates.lines() {
        let mut update: Vec<&str> = update.split(",").collect();
        let mut is_invalid_update = false;
        for i in 1..update.len() {
            if let Some(before) = rule_map.get(update[i]) {
                for j in 0..i {
                    if before.contains(update[j]) {
                        update.swap(i, j);
                        is_invalid_update = true;
                    }
                }
            }
        }

        if is_invalid_update {
            //println!("{:?}", update);
            let middle: i32 = update[update.len() / 2].parse().unwrap();
            result += middle;
        }
    }

    println!("{}", result);
}

fn main() {
    let input = "src/inputs/day5.txt";

    match std::env::args().nth(1).as_deref() {
        Some("1") => part1(input),
        Some("2") => part2(input),
        Some("12") | Some("21") | None => {
            part1(input);
            part2(input);
        }
        _ => panic!("Invalid arg. Pass in 1, 2 or leave empty for both."),
    }
}
