fn part1(input: &str) -> i64 {
    let mut sum = 0i64;

    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();

        for n in start..=end {
            let s = n.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let mid = s.len() / 2;
            if s[..mid] == s[mid..] {
                sum += n;
            }
        }
    }

    sum
}

fn part2(input: &str) -> i64 {
    let mut sum = 0i64;

    for range in input.trim().split(",") {
        let (start, end) = range.split_once("-").unwrap();
        let start: i64 = start.parse().unwrap();
        let end: i64 = end.parse().unwrap();

        for n in start..=end {
            let s1 = n.to_string();
            let s2 = s1.clone() + &s1;
            if s2[1..s2.len() - 1].contains(&s1) {
                sum += n;
            }
        }
    }

    sum
}

fn main() {
    let input = std::fs::read_to_string("input/day2.txt").unwrap();

    let now = std::time::Instant::now();
    let res = part2(&input);
    let time = now.elapsed();

    println!("{res}\npart 2 in {time:.2?}");
}
