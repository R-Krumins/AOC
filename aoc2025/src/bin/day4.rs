const EMPTY: u8 = b'.';
const ROLL: u8 = b'@';

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

pub fn part1(input: &str) -> u64 {
    let h = input.lines().count() + 2;
    let w = input.lines().next().unwrap().len() + 2;

    let mut grid: Vec<u8> = vec![EMPTY; h * w];
    input.lines().enumerate().for_each(|(i, row)| {
        let from = h * (i + 1) + 1;
        let to = from + w - 2;
        grid[from..to].copy_from_slice(row.as_bytes());
    });

    let mut accesable_rolls = 0;
    for y in 1..w - 1 {
        for x in 1..h - 1 {
            if grid[y * w + x] != ROLL {
                continue;
            }

            let mut roll_count = 0;

            for (dx, dy) in OFFSETS {
                let x = x.wrapping_add_signed(dx);
                let y = y.wrapping_add_signed(dy);
                if grid[y * w + x] == ROLL {
                    roll_count += 1;
                }
            }

            if roll_count < 4 {
                accesable_rolls += 1;
            }
        }
    }
    accesable_rolls
}

pub fn part2(input: &str) -> u64 {
    let h = input.lines().count() + 2;
    let w = input.lines().next().unwrap().len() + 2;

    let mut grid: Vec<u8> = vec![EMPTY; h * w];
    input.lines().enumerate().for_each(|(i, row)| {
        let from = h * (i + 1) + 1;
        let to = from + w - 2;
        grid[from..to].copy_from_slice(row.as_bytes());
    });

    let mut removed_rolls = 0;
    loop {
        let mut no_rolls_removed = true;
        for y in 1..w - 1 {
            for x in 1..h - 1 {
                if grid[y * w + x] != ROLL {
                    continue;
                }

                let mut roll_count = 0;

                for (dx, dy) in OFFSETS {
                    let x = x.wrapping_add_signed(dx);
                    let y = y.wrapping_add_signed(dy);
                    if grid[y * w + x] == ROLL {
                        roll_count += 1;
                    }
                }

                if roll_count < 4 {
                    grid[y * w + x] = EMPTY;
                    removed_rolls += 1;
                    no_rolls_removed = false;
                }
            }
        }

        if no_rolls_removed {
            break;
        }
    }
    removed_rolls
}

fn main() {
    let input = std::fs::read_to_string("input/day4.txt").unwrap();

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
