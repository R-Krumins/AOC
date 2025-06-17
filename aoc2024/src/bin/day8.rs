use std::collections::{HashMap, HashSet};
use std::fs;

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let mut x = 0;
    let mut max_x = 0;
    let mut y = 0;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Node {
        x: i32,
        y: i32,
    }

    let mut nodes: HashMap<char, Vec<Node>> = HashMap::new();

    for ch in file_content.chars() {
        match ch {
            '.' => x += 1,
            '\n' => {
                y += 1;
                max_x = x;
                x = 0; // reset x when moving to a new line
            }
            _ => {
                nodes.entry(ch).or_insert_with(Vec::new).push(Node { x, y });
                x += 1;
            }
        }
    }

    let width = max_x;
    let height = y;
    let mut antinodes: HashSet<Node> = HashSet::new();

    for (_, n) in &nodes {
        for a in n {
            for b in n {
                if std::ptr::eq(a, b) {
                    continue;
                }

                let cx = 2 * b.x - a.x;
                let cy = 2 * b.y - a.y;

                if cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes.insert(Node { x: cx, y: cy });
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    let mut x = 0;
    let mut max_x = 0;
    let mut y = 0;

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct Node {
        x: i32,
        y: i32,
    }

    let mut nodes: HashMap<char, Vec<Node>> = HashMap::new();

    for ch in file_content.chars() {
        match ch {
            '.' => x += 1,
            '\n' => {
                y += 1;
                max_x = x;
                x = 0; // reset x when moving to a new line
            }
            _ => {
                nodes.entry(ch).or_insert_with(Vec::new).push(Node { x, y });
                x += 1;
            }
        }
    }

    let width = max_x;
    let height = y;
    let mut antinodes: HashSet<Node> = HashSet::new();

    for (_, n) in &nodes {
        for a in n {
            for b in n {
                if std::ptr::eq(a, b) {
                    continue;
                }

                antinodes.insert(Node { x: a.x, y: a.y });
                antinodes.insert(Node { x: b.x, y: b.y });

                let delta_x = b.x - a.x;
                let delta_y = b.y - a.y;

                let mut cx = b.x + delta_x;
                let mut cy = b.y + delta_y;

                while cx >= 0 && cx < width && cy >= 0 && cy < height {
                    antinodes.insert(Node { x: cx, y: cy });
                    cx += delta_x;
                    cy += delta_y;
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn main() {
    let input = "src/inputs/day8.txt";

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
