use std::fs;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

struct Vec2 {
    x: usize,
    y: usize,
}

impl Direction {
    fn add(&self, v: &Vec2, max_x: usize, max_y: usize) -> Option<Vec2> {
        match self {
            Direction::Up => {
                if v.y > 0 {
                    Some(Vec2 { x: v.x, y: v.y - 1 })
                } else {
                    None
                }
            }
            Direction::Down => {
                if v.y + 1 < max_y {
                    Some(Vec2 { x: v.x, y: v.y + 1 })
                } else {
                    None
                }
            }
            Direction::Left => {
                if v.x > 0 {
                    Some(Vec2 { x: v.x - 1, y: v.y })
                } else {
                    None
                }
            }
            Direction::Right => {
                if v.x + 1 < max_x {
                    Some(Vec2 { x: v.x + 1, y: v.y })
                } else {
                    None
                }
            }
        }
    }

    fn turn_right(&mut self) {
        *self = match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines: Vec<&str> = file_content.lines().collect();

    let map_height = lines.len();
    let map_width = lines[0].len();

    println!("map width:{} height:{}", map_width, map_height);

    let mut guard = Vec2 { x: 0, y: 0 };
    let mut move_dir = Direction::Up;

    let mut map: Vec<Vec<u8>> = lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    '.' => 0,
                    '#' => 1,
                    '^' | 'v' | '<' | '>' => {
                        guard.x = x;
                        guard.y = y;
                        3
                    }
                    _ => panic!(),
                })
                .collect()
        })
        .collect();

    let mut pos_count = 0;
    loop {
        if let Some(new_guard) = move_dir.add(&guard, map_width, map_height) {
            let pos = map[new_guard.y][new_guard.x];

            if pos == 0 {
                pos_count += 1;
                map[new_guard.y][new_guard.x] = 3;
                guard = new_guard;
            } else if pos == 3 {
                guard = new_guard;
            }

            if pos == 1 {
                move_dir.turn_right();
                //println!("turned right and now facing {:?}", move_dir);
            }
        } else {
            pos_count += 1;
            break;
        }
    }

    println!("{}", pos_count);
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();
    let lines = file_content.split("\n");
}

fn main() {
    let input = "src/inputs/day6.txt";

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
