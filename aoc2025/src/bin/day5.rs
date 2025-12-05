pub fn part1(input: &str) -> u64 {
    let (ranges, ingredients) = input.split_once("\n\n").unwrap();
    let mut ranges: Vec<(u64, u64)> = ranges
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|r| r.0);
    ranges = ranges
        .into_iter()
        .fold(Vec::<(u64, u64)>::new(), |mut acc, r| {
            if let Some(last) = acc.last_mut()
                && r.0 <= last.1
            {
                last.1 = last.1.max(r.1);
            } else {
                acc.push(r);
            }
            acc
        });
    ingredients
        .lines()
        .filter(|l| {
            let n: u64 = l.parse().unwrap();
            ranges.iter().any(|(min, max)| n >= *min && n <= *max)
        })
        .count() as u64
}

pub fn part2(input: &str) -> u64 {
    let mut ranges: Vec<(u64, u64)> = input
        .split_once("\n\n")
        .unwrap()
        .0
        .lines()
        .map(|range| {
            let (min, max) = range.split_once("-").unwrap();
            (min.parse().unwrap(), max.parse().unwrap())
        })
        .collect();
    ranges.sort_by_key(|r| r.0);
    ranges
        .into_iter()
        .fold(Vec::<(u64, u64)>::new(), |mut acc, r| {
            if let Some(last) = acc.last_mut()
                && r.0 <= last.1
            {
                last.1 = last.1.max(r.1);
            } else {
                acc.push(r);
            }
            acc
        })
        .iter()
        .map(|(min, max)| max - min + 1)
        .sum()
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
