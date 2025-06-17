use std::fs;
pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.lines();

    let mut safe_report_count = 0;

    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if line.is_empty() {
            continue;
        }

        let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();

        let must_increase: bool = diffs[0] > 0;
        let is_safe_report = diffs.iter().all(|&diff| {
            (must_increase && diff >= 1 && diff <= 3)
                || (!must_increase && diff <= -1 && diff >= -3)
        });

        if is_safe_report {
            safe_report_count += 1;
        }
    }

    println!("{:?}", safe_report_count);
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.lines();

    let mut safe_report_count = 0;

    for line in lines {
        let report: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect();

        if line.is_empty() {
            continue;
        }

        let diffs: Vec<i32> = report.windows(2).map(|w| w[1] - w[0]).collect();
        let mut must_increase: bool = (diffs[0] + diffs[1]) > 0;

        let mut is_safe_report = report_safety(&diffs, must_increase);

        if !is_safe_report {
            let mut variant = diffs.clone();
            variant.remove(0);
            must_increase = (variant[0] + variant[1]) > 0;
            is_safe_report = report_safety(&variant, must_increase);
        }

        if !is_safe_report {
            for i in 0..diffs.len() - 1 {
                let mut variant = diffs.clone();
                variant[i] += variant[i + 1];
                variant.remove(i + 1);
                must_increase = (variant[0] + variant[1]) > 0;
                is_safe_report = report_safety(&variant, must_increase);
                if is_safe_report {
                    break;
                }
            }
        }

        if !is_safe_report {
            let mut variant = diffs.clone();
            variant.pop();
            must_increase = (variant[0] + variant[1]) > 0;
            is_safe_report = report_safety(&variant, must_increase);
        }

        if is_safe_report {
            safe_report_count += 1;
        }
    }

    println!("{:?}", safe_report_count);
}

fn report_safety(diffs: &Vec<i32>, must_increase: bool) -> bool {
    diffs.iter().all(|&diff| {
        (must_increase && diff >= 1 && diff <= 3) || (!must_increase && diff <= -1 && diff >= -3)
    })
}

fn main() {
    let input = "src/inputs/day2.txt";

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
