use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn _part1() {
    let file_content = fs::read_to_string("src/inputs/day5.txt").unwrap();
    let (rules, updates) = file_content.split_once("\n\n").unwrap();

    //value: number, key: must be before
    let mut rule_map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for rule in rules.lines() {
        let (n1, n2) = rule.split_once("|").unwrap();

        rule_map.entry(n1).or_insert_with(HashSet::new).insert(n2);
    }

    println!("{:?}", rule_map);

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

pub fn _part2() {
    let file_content = fs::read_to_string("src/inputs/day5.txt").unwrap();
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
