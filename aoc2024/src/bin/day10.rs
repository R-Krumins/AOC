use std::collections::HashSet;
use std::fs;

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let height = file_content.matches('\n').count();
    let width = file_content.split_once('\n').unwrap().0.len();
    //println!("{width}x{height}");

    // Parse into 2d u8 vec
    let mut map = vec![vec![0u8; width]; height];
    for (y, line) in file_content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map[y][x] = ch.to_string().parse::<u8>().unwrap();
        }
    }

    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                let mut visited_nines = HashSet::new();
                dfs(&map, x, y, 0, &mut visited_nines);
                score += visited_nines.len();
            }
        }
    }

    println!("{score}");
}

fn dfs(
    map: &Vec<Vec<u8>>,
    x: usize,
    y: usize,
    expected: u8,
    visited_nines: &mut HashSet<(usize, usize)>,
) {
    let width = map[0].len();
    let height = map.len();

    if x >= width || y >= height {
        return;
    }

    let current = map[y][x];

    if current != expected {
        return;
    }

    if current == 9 {
        visited_nines.insert((x, y));
        return;
    }

    let next_expected = expected + 1;
    if x > 0 {
        dfs(map, x - 1, y, next_expected, visited_nines);
    }
    if y > 0 {
        dfs(map, x, y - 1, next_expected, visited_nines);
    }
    dfs(map, x, y + 1, next_expected, visited_nines);
    dfs(map, x + 1, y, next_expected, visited_nines);
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let height = file_content.matches('\n').count();
    let width = file_content.split_once('\n').unwrap().0.len();
    //println!("{width}x{height}");

    // Parse into 2d u8 vec
    let mut map = vec![vec![0u8; width]; height];
    for (y, line) in file_content.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            map[y][x] = ch.to_string().parse::<u8>().unwrap();
        }
    }

    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 0 {
                score += dfs2(&map, x, y, 0);
            }
        }
    }

    println!("{score}");
}

fn dfs2(map: &Vec<Vec<u8>>, x: usize, y: usize, expected: u8) -> u32 {
    let width = map[0].len();
    let height = map.len();

    if x >= width || y >= height {
        return 0;
    }

    let current = map[y][x];

    if current != expected {
        return 0;
    }

    if current == 9 {
        return 1;
    }

    let mut score = 0;
    let next_expected = expected + 1;
    if x > 0 {
        score += dfs2(map, x - 1, y, next_expected);
    }
    if y > 0 {
        score += dfs2(map, x, y - 1, next_expected);
    }
    score += dfs2(map, x, y + 1, next_expected);
    score += dfs2(map, x + 1, y, next_expected);

    return score;
}
fn main() {
    let input = "src/inputs/day10.txt";

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
