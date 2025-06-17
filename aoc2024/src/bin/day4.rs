use regex::Regex;
use std::fs;

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let lines: Vec<&str> = file_content.lines().collect();

    let mut colums: Vec<String> = vec![String::new(); lines.len()];
    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            colums[i].push(ch);
        }
    }

    let mut diognals_pos: Vec<String> = vec![String::new(); lines.len() + colums.len() - 1];
    for (i, line) in lines.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            diognals_pos[i + j].push(ch);
        }
    }

    let mut diognals_neg: Vec<String> = vec![String::new(); lines.len() + colums.len() - 1];
    for (i, line) in lines.iter().enumerate() {
        let n = lines.len() - 1;
        for (j, ch) in line.chars().enumerate() {
            diognals_neg[n + j - i].push(ch);
        }
    }

    let re1 = Regex::new(r"XMAS").unwrap();
    let re2 = Regex::new(r"SAMX").unwrap();

    let mut result: usize = 0;

    for line in &lines {
        result += re1.captures_iter(line).count();
        result += re2.captures_iter(line).count();
    }

    for col in &colums {
        result += re1.captures_iter(col).count();
        result += re2.captures_iter(col).count();
    }

    for diognal in &diognals_pos {
        result += re1.captures_iter(diognal).count();
        result += re2.captures_iter(diognal).count();
    }

    for diognal in &diognals_neg {
        result += re1.captures_iter(diognal).count();
        result += re2.captures_iter(diognal).count();
    }

    println!("{}", result)
}

pub fn part2(input: &str) {
    let text = fs::read_to_string(input).unwrap();

    let l: usize = text.lines().next().map(|x| x.len()).unwrap();
    let text = text.replace("\n", "");
    let n: usize = text.len();

    let mut result: i32 = 0;

    for i in l..n - l - 1 {
        if text.chars().nth(i).unwrap() == 'A' {
            if i % l == 0 || i % l == l - 1 {
                continue;
            }
            let tl: char = text.chars().nth(i - l - 1).unwrap();
            let tr: char = text.chars().nth(i - l + 1).unwrap();
            let bl: char = text.chars().nth(i + l - 1).unwrap();
            let br: char = text.chars().nth(i + l + 1).unwrap();
            let diognal1 = (tr == 'M' && bl == 'S') || (tr == 'S' && bl == 'M');
            let diognal2 = (tl == 'M' && br == 'S') || (tl == 'S' && br == 'M');
            if diognal1 && diognal2 {
                result += 1;
            }
        }
    }

    println!("{}", result);
}

fn main() {
    let input = "src/inputs/day4.txt";

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
