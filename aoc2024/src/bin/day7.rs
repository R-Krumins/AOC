use std::fs;

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.split("\n");

    let mut sum = 0;
    for line in lines {
        if let Some((k, v)) = line.split_once(":") {
            let target = k.parse::<u64>().unwrap();
            let numbs: Vec<u64> = v
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            if dfs(&numbs, 0, 0, target) {
                sum += target;
            }
        }
    }

    println!("{}", sum);
}

fn dfs(numbs: &Vec<u64>, index: usize, current: u64, target: u64) -> bool {
    if index == numbs.len() {
        return current == target;
    }

    // Try adding
    if dfs(numbs, index + 1, current + numbs[index], target) {
        return true;
    }

    // Try multiplying
    if dfs(numbs, index + 1, current * numbs[index], target) {
        return true;
    }

    return false;
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.split("\n");

    let mut sum = 0;
    for line in lines {
        if let Some((k, v)) = line.split_once(":") {
            let target = k.parse::<u64>().unwrap();
            let numbs: Vec<u64> = v
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();

            if dfs2(&numbs, 0, 0, target) {
                sum += target;
            }
        }
    }

    println!("{}", sum);
}

fn dfs2(numbs: &Vec<u64>, index: usize, current: u64, target: u64) -> bool {
    if index == numbs.len() {
        return current == target;
    }

    let next = numbs[index];

    // Try adding
    if dfs2(numbs, index + 1, current + next, target) {
        return true;
    }

    // Try multiplying
    if dfs2(numbs, index + 1, current * next, target) {
        return true;
    }

    // Try concating
    let concat = format!("{}{}", current, next).parse::<u64>().unwrap();
    if dfs2(numbs, index + 1, concat, target) {
        return true;
    }

    return false;
}

fn main() {
    let input = "src/inputs/day7.txt";

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
