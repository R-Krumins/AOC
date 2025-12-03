pub fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|bank| {
            let mut n1 = 0;
            let mut idx = 0;
            for i in 0..bank.len() - 1 {
                if bank[i] > n1 {
                    n1 = bank[i];
                    idx = i;
                }
            }
            let n2 = bank[idx + 1..].iter().max().unwrap();
            (n1 * 10 + n2) as u64
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .map(|bank| {
            let mut joltage = 0;
            let mut start = 0;
            for i in (0..12).rev() {
                let mut max = 0;
                for j in start..bank.len() - i {
                    if bank[j] > max {
                        max = bank[j];
                        start = j + 1;
                    }
                }
                joltage += (max as u64) * 10u64.pow(i as u32);
            }
            joltage
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day3.txt").unwrap();

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
