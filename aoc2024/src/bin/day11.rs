use std::collections::HashMap;
use std::fs;
use std::time::Instant;

type SMap = HashMap<u64, u64>;

fn split(stone: u64) -> Option<(u64, u64)> {
    let stone = stone.to_string();

    if stone.len() % 2 != 0 {
        return None;
    }

    let mid = stone.len() / 2;
    let left = stone[..mid].parse::<u64>().unwrap();
    let right = stone[mid..].parse::<u64>().unwrap();

    return Some((left, right));
}

fn blink(stones: SMap) -> SMap {
    let mut new_stones: SMap = HashMap::with_capacity(stones.len());

    for (stone, count) in stones {
        if stone == 0 {
            *new_stones.entry(1).or_insert(0) += count;
            continue;
        }

        if let Some((l, r)) = split(stone) {
            *new_stones.entry(l).or_insert(0) += count;
            *new_stones.entry(r).or_insert(0) += count;
            continue;
        }

        *new_stones.entry(stone * 2024).or_insert(0) += count;
    }

    return new_stones;
}
pub fn solve(input: &str, blink_count: usize) {
    let start = Instant::now();

    let file_content = fs::read_to_string(input).unwrap();
    let initial: Vec<u64> = file_content
        .trim()
        .split(" ")
        .map(|x| x.to_string().parse::<u64>().unwrap())
        .collect();

    let mut stones: SMap = HashMap::new();
    for stone in &initial {
        *stones.entry(*stone).or_insert(0) += 1;
    }

    for _ in 0..blink_count {
        stones = blink(stones);
    }

    let mut total = 0;
    for count in stones.values() {
        total += count;
    }

    let duration = start.elapsed();
    println!("{total}");
    println!("in {duration:?}");
}

fn main() {
    let input = "src/inputs/day11.txt";

    match std::env::args().nth(1).as_deref() {
        Some("1") => solve(input, 25),
        Some("2") => solve(input, 75),
        Some("12") | Some("21") | None => {
            solve(input, 25);
            solve(input, 75);
        }
        _ => panic!("Invalid arg. Pass in 1, 2 or leave empty for both."),
    }
}
