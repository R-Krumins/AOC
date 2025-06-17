use regex::Regex;
use std::fs;

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(file_content.as_str())
        .map(|caps| {
            let num1: i32 = caps[1].parse().unwrap();
            let num2: i32 = caps[2].parse().unwrap();
            num1 * num2
        })
        .sum();

    println!("{}", result);
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|don't\(\)|do\(\)").unwrap();

    let mut take = true;
    let result: i32 = re
        .captures_iter(file_content.as_str())
        .filter(|m| match &m[0] {
            "do()" => {
                take = true;
                false
            }
            "don't()" => {
                take = false;
                false
            }
            _ => take,
        })
        .map(|m| {
            let num1: i32 = m[1].parse().unwrap();
            let num2: i32 = m[2].parse().unwrap();
            num1 * num2
        })
        .sum();

    println!("{}", result);
}

fn main() {
    let input = "src/inputs/day3.txt";

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
