#! /bin/bash

# ----------------------
# This is a little script for automaticly setting up each
# AOC day and fetching puzzle input
# ---------------------

if [ -z "$1" ]; then
  echo "Error: Must provide day argument!"
  exit 1
fi

day=$1
filename="src/bin/day${day}.rs"

if find "./src/bin/" -maxdepth 1 -type f -name "day${day}.rs" | grep -q .; then
  echo "Error: ${filename} already exists!"
  exit 1
fi

# GET PUZZLE INPUT
source .env
curl --cookie "session=${AOC_SESSION_TOKEN}" https://adventofcode.com/2024/day/${day}/input > "src/inputs/day${day}.txt"
echo "Fetched puzzle input"

# CREATE day.rs FILE
cat <<EOF > $filename
use std::fs;
use std::time::Instant;

pub fn part1(input: &str) -> u64 {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.split("\n");

    return 0;
}

pub fn part2(input: &str) -> u64 {
    let file_content = fs::read_to_string(input).unwrap();
    let _lines = file_content.split("\n");

    return 0;
}

fn main() {
    let input = "src/inputs/day${DAY}.txt";

    let (do_part1, do_part2) = match std::env::args().nth(1).as_deref() {
        Some("1") => (true, false),
        Some("2") => (false, true),
        Some("12") | Some("21") | None => (true, true),
        _ => panic!("Invalid arg. Pass in 1, 2 or leave empty for both."),
    };

    if do_part1 {
        let start = Instant::now();
        let res = part1(input);
        let duration = start.elapsed();
        println!("{res}");
        println!("in {duration:?}");
    }

    if do_part2 {
        let start = Instant::now();
        let res = part2(input);
        let duration = start.elapsed();
        println!("{res}");
        println!("in {duration:?}");
    }
}
EOF
echo "Created ${filename}"

