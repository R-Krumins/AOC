use std::{collections::HashMap, fs};

pub fn part1() {
    let file_content = fs::read_to_string("src/inputs/day1.txt").unwrap();
    let lines = file_content.split("\n");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        if let Some((n1, n2)) = line.split_once("   ") {
            left.push(n1.parse::<i32>().unwrap());
            right.push(n2.parse::<i32>().unwrap());
        }
    }
    left.sort();
    right.sort();
    let result: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{}", result);
}

pub fn part2() {
    let file_content = fs::read_to_string("src/inputs/day1.txt").unwrap();
    let lines = file_content.split("\n");
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    for line in lines {
        if let Some((n1, n2)) = line.split_once("   ") {
            left.push(n1.parse::<i32>().unwrap());
            right.push(n2.parse::<i32>().unwrap());
        }
    }

    let mut right_counts: HashMap<i32, i32> = HashMap::new();
    for n in right {
        *right_counts.entry(n).or_insert(0) += 1;
    }

    let result: i32 = left
        .iter()
        .map(|n| n * right_counts.get(n).unwrap_or(&0))
        .sum();

    println!("{}", result);
}
