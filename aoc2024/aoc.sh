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
filename="src/day${day}.rs"

if find "./src/" -maxdepth 1 -type f -name "day${day}.rs" | grep -q .; then
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

pub fn part1() {
  let file_content = fs::read_to_string("src/inputs/day${day}.txt").unwrap();
  let lines = file_content.split("\n");
 
}

pub fn part2() {
  let file_content = fs::read_to_string("src/inputs/day${day}.txt").unwrap();
  let lines = file_content.split("\n");
}
EOF
echo "Created ${filename}"

# UDPATE MAIN.RS
sed -i -E "s/day[0-9]+/day${day}/g" ./src/main.rs
echo "Updated main.rs"

echo "FINISHED"
