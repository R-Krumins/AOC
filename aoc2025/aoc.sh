#! /bin/bash

# ----------------------
# This is a little script for automaticly setting up each
# AOC day and fetching puzzle input
# ---------------------

if [ -z "$1" ]; then
  echo "Error: Must provide day argument!"
  exit 1
fi

YEAR=2025
day=$1
filename="src/bin/day${day}.rs"

if find "./src/bin/" -maxdepth 1 -type f -name "day${day}.rs" | grep -q .; then
  echo "Error: ${filename} already exists!"
  exit 1
fi

# GET PUZZLE INPUT
source .env
curl --cookie "session=${AOC_SESSION_TOKEN}" https://adventofcode.com/${YEAR}/day/${day}/input > "src/inputs/day${day}.txt"
echo "Fetched puzzle input"

# CREATE day.rs FILE
cat <<EOF > $filename
pub fn part1(input: &str) -> u64 {
	todo!()
}

pub fn part2(input: &str) -> u64 {
	todo!()
}

fn main() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();

    let (do_part1, do_part2) = match std::env::args().nth(1).as_deref() {
        Some("1") => (true, false),
        Some("2") => (false, true),
        Some("12") | Some("21") | None => (true, true),
        Some(_) => panic!("Invalid part arg. Expected 1, 2 or empty for both"),
    };

    if do_part1 {
        let now = std::time::Instant::now();
        let res = part1(&input);
        let time = now.elapsed();
        println!("{res}\npart 1 in {time:.2?}");
    }

    if do_part2 {
        let now = std::time::Instant::now();
        let res = part2(&input);
        let time = now.elapsed();
        println!("{res}\npart 2 in {time:.2?}");
    }
}
EOF
echo "Created ${filename}"

