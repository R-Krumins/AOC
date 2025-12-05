use std::cmp::max;
use std::ops::RangeInclusive;

pub fn part1(input: &str) -> u64 {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();

    let ranges: Vec<(usize, usize)> = ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            let min = min.parse::<usize>().unwrap();
            let max = max.parse::<usize>().unwrap();
            (min, max)
        })
        .collect();

    let mut fresh_count = 0;
    for ingr in ingredients.lines() {
        let ingr = ingr.parse::<usize>().unwrap();

        for r in &ranges {
            if ingr >= r.0 && ingr <= r.1 {
                fresh_count += 1;
                break;
            }
        }
    }

    fresh_count
}

pub fn part2(input: &str) -> u64 {
    #[derive(Clone, Copy, Debug)]
    struct Range {
        min: u64,
        max: u64,
    }

    let (ranges, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<Range> = ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            let min = min.parse().unwrap();
            let max = max.parse().unwrap();
            Range { min, max }
        })
        .collect();

    ranges.sort_by_key(|r| r.min);

    let mut merged = vec![ranges[0]];

    for r in ranges.into_iter().skip(1) {
        let last = merged.last_mut().unwrap();

        if r.min <= last.max {
            last.max = max(last.max, r.max);
        } else {
            merged.push(r);
        }
    }

    let mut fresh_count = 0u64;
    for r in merged {
        fresh_count += r.max - r.min + 1;
    }

    fresh_count
}

fn main() {
    let input = std::fs::read_to_string("input/day5.txt").unwrap();

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
