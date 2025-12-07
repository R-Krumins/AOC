#[derive(Debug)]
enum Ops {
    Mult,
    Add,
}
pub fn part1(input: &str) -> u64 {
    let input: Vec<&str> = input.lines().collect();
    let ops: Vec<Ops> = input[input.len() - 1]
        .chars()
        .filter_map(|ch| match ch {
            '*' => Some(Ops::Mult),
            '+' => Some(Ops::Add),
            _ => None,
        })
        .collect();

    let len = input.len();
    let numbs: Vec<Vec<u64>> = input
        .into_iter()
        .take(len - 1)
        .map(|line| {
            line.split(" ")
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let problem_count = numbs[0].len();
    let problem_len = numbs.len();
    let mut problems: Vec<Vec<u64>> = vec![vec![0; problem_len]; problem_count];

    for len in 0..problem_len {
        for count in 0..problem_count {
            problems[count][len] = numbs[len][count];
        }
    }

    problems
        .into_iter()
        .zip(ops)
        .map(|(numbs, op)| {
            numbs
                .into_iter()
                .reduce(|acc, n| match op {
                    Ops::Mult => acc * n,
                    Ops::Add => acc + n,
                })
                .unwrap()
        })
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let w = input.iter().position(|c| *c == b'\n').unwrap() + 1;
    let h = input.split(|c| *c == b'\n').count() - 1;
    let mut op = input[w * (h - 1)];
    let mut col = vec![0u8; h - 1];
    let mut final_res = 0u64;
    let mut equation_res = 0u64;

    for x in 0..w - 1 {
        // scan column
        for y in 0..h - 1 {
            col[y] = input[y * w + x];
        }

        // check if column ends current equation
        if col.iter().all(|c| *c == b' ') {
            final_res += equation_res;
            equation_res = 0;
            op = input[(h - 1) * w + (x + 1)];
            continue;
        }

        // combine digits into a number
        let mut numb = 0u64;
        for y in 0..h - 1 {
            if col[y] != b' ' {
                let digit = col[y] - b'0';
                numb = numb * 10 + digit as u64;
            }
        }

        // add number to current equation result
        if equation_res == 0 {
            equation_res = numb;
        } else {
            match op {
                b'*' => equation_res *= numb,
                b'+' => equation_res += numb,
                _ => unreachable!(),
            }
        }
    }

    final_res + equation_res
}

fn main() {
    let input = std::fs::read_to_string("input/day6.txt").unwrap();

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
