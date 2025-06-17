use std::fs;

type Disk = Vec<Option<u32>>;
pub fn part1(input: &str) {
    let file_content = fs::read_to_string(input).unwrap();

    // statistical guess of disk size assuming the average block size is 5
    let disk_size: usize = file_content.chars().count() * 5;
    let mut disk: Disk = Vec::with_capacity(disk_size);

    parse_disk(file_content.as_str(), &mut disk);
    //print_disk(&disk);
    compact_disk(&mut disk);
    //print_disk(&disk);

    println!("{}", calc_disk_checksum(&disk));
}

fn parse_disk(text: &str, disk: &mut Disk) {
    let mut i = 0;
    let mut is_file = true;
    for ch in text.chars() {
        if ch == '\n' {
            break;
        }
        let digit = ch.to_digit(10).unwrap();

        for _ in 0..digit {
            if is_file {
                disk.push(Some(i));
            } else {
                disk.push(None);
            }
        }
        if is_file {
            i += 1;
        }
        is_file = !is_file;
    }
}

#[allow(dead_code)]
fn print_disk(disk: &Disk) {
    for x in disk {
        match x {
            Some(d) => print!("{}", d),
            None => print!("."),
        }
    }
    println!();
}

fn compact_disk(disk: &mut Disk) {
    let mut j = disk.len() - 1;
    for i in 0..disk.len() - 1 {
        if disk[i] == None {
            loop {
                if j <= i {
                    return;
                }
                if let Some(d) = disk[j] {
                    disk[i] = Some(d);
                    disk[j] = None;
                    break;
                }
                j -= 1;
            }
        }
    }
}

fn calc_disk_checksum(disk: &Disk) -> u64 {
    let mut checksum: u64 = 0;
    for i in 0..disk.len() - 1 {
        if let Some(d) = disk[i] {
            checksum += (i as u64) * (d as u64);
        } else {
            return checksum;
        }
    }
    return checksum;
}

enum Block {
    File { id: usize, size: usize },
    Free(usize),
}

pub fn part2(input: &str) {
    let file_content = fs::read_to_string(input).unwrap().trim().to_string();

    let mut disk: Vec<Block> = file_content
        .chars()
        .enumerate()
        .map(|(i, ch)| {
            let size = ch.to_digit(10).unwrap() as usize;
            if i % 2 == 0 {
                let id = i - i / 2;
                Block::File { id, size }
            } else {
                Block::Free(size)
            }
        })
        .collect();

    //print_blocks(&disk);

    // reorder blocks
    for j in (0..disk.len()).rev() {
        let Block::File {
            id,
            size: file_size,
        } = disk[j]
        else {
            continue;
        };

        for i in 0..j {
            let Block::Free(free_size) = disk[i] else {
                continue;
            };

            if file_size <= free_size {
                disk[i] = Block::File {
                    id,
                    size: file_size,
                };
                disk[j] = Block::Free(file_size);

                if free_size > file_size {
                    disk.insert(i + 1, Block::Free(free_size - file_size));
                    // Note: this insertion shifts indices, so you might need to adjust j
                }

                break;
            }
        }
    }

    //print_blocks(&disk);

    let mut checksum = 0;
    let mut pos = 0;
    for block in &disk {
        match block {
            Block::File { id, size } => {
                for _ in 0..*size {
                    checksum += id * pos;
                    pos += 1;
                }
            }
            Block::Free(size) => {
                pos += *size;
            }
        }
    }

    println!("{checksum}");
}

fn main() {
    let input = "src/inputs/day9.txt";

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
