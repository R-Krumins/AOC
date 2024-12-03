use regex::Regex;
use std::fs;

pub fn _part1() {
    let file_content = fs::read_to_string("src/inputs/day3.txt").unwrap();

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

pub fn _part2() {
    let file_content = fs::read_to_string("src/inputs/day3.txt").unwrap();

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
