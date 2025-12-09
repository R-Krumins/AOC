const SPLITTER: u8 = b'^';
const START: u8 = b'S';
const BEAM: u8 = b'|';

pub fn part1(input: &str) -> u64 {
    let w = input.chars().position(|b| b == '\n').unwrap() + 1;
    let h = input.lines().count();
    let mut manifold = input.as_bytes().to_vec();
    let mut split_count = 0;

    // place first beam
    let start_idx = manifold.iter().position(|b| *b == START).unwrap();
    manifold[1 * w + start_idx] = BEAM;

    for y in 1..h - 1 {
        for x in 0..w - 1 {
            let this_idx = y * w + x;
            let below_idx = this_idx + w;
            if manifold[this_idx] == BEAM {
                if manifold[below_idx] == SPLITTER {
                    manifold[below_idx + 1] = BEAM;
                    manifold[below_idx - 1] = BEAM;
                    split_count += 1;
                } else {
                    manifold[below_idx] = BEAM;
                }
            }
        }
    }
    split_count
}

// TOO SLOW
// pub fn part2(input: &str) -> u64 {
//     let w = input.find('\n').unwrap() + 1;
//     let manifold = input.as_bytes();
//
//     fn travel(x: usize, y: usize, w: usize, m: &[u8]) -> u64 {
//         match m.get((y + 1) * w + x) {
//             Some(&cell) => match cell {
//                 SPLITTER => travel(x + 1, y, w, m) + travel(x - 1, y, w, m),
//                 EMPTY => travel(x, y + 1, w, m),
//                 _ => unreachable!(),
//             },
//             None => 1,
//         }
//     }
//
//     let start = input.find('S').unwrap();
//     travel(start, 1, w, manifold)
// }

pub fn part2(input: &str) -> u64 {
    let w = input.find('\n').unwrap() + 1;
    let h = input.lines().count();
    let manifold = input.as_bytes();
    let mut quantum_manifold = vec![0u64; manifold.len()];

    // place first beam
    let start = input.find('S').unwrap();
    quantum_manifold[1 * w + start] = 1;

    for y in 1..h - 1 {
        for x in 0..w - 1 {
            let this_idx = y * w + x;
            let below_idx = this_idx + w;
            if quantum_manifold[this_idx] > 0 {
                if manifold[below_idx] == SPLITTER {
                    quantum_manifold[below_idx + 1] += quantum_manifold[this_idx];
                    quantum_manifold[below_idx - 1] += quantum_manifold[this_idx];
                } else {
                    quantum_manifold[below_idx] += quantum_manifold[this_idx];
                }
            }
        }
    }

    quantum_manifold[w * (h - 1) + 0..].iter().sum()
}

fn main() {
    let input = std::fs::read_to_string("input/day7.txt").unwrap();

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
