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

pub fn part1(input: &str) {
  let file_content = fs::read_to_string(input).unwrap();
  let lines = file_content.split("\n");
 
}

pub fn part2(input: &str) {
  let file_content = fs::read_to_string(input).unwrap();
  let lines = file_content.split("\n");
}

fn main() {
    let input = "src/inputs/day${day}.txt";

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
EOF
echo "Created ${filename}"

