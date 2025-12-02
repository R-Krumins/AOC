fn part1(input: &str) -> i64 {
    let mut zero_count = 0i64;
    let mut rot_pos = 50i32;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let rotation = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            rot => panic!("unexpected rotation: {rot}"),
        };
        let amount: i32 = line[1..].parse().unwrap();

        rot_pos = (rot_pos + amount * rotation).rem_euclid(100);
        if rot_pos == 0 {
            zero_count += 1;
        }
    }
    return zero_count;
}

fn part2(input: &str) -> i64 {
    let mut zero_count = 0i64;
    let mut rot_pos = 50i64;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let rotation = match &line[0..1] {
            "L" => -1,
            "R" => 1,
            rot => panic!("unexpected rotation: {rot}"),
        };
        let amount: i64 = line[1..].parse().unwrap();

        rot_pos += amount * rotation;

        if rot_pos <= 0 && rot_pos - amount * rotation != 0 {
            zero_count += 1;
        }
        zero_count += rot_pos.abs() / 100;

        rot_pos = rot_pos.rem_euclid(100);
    }
    return zero_count;
}

fn main() {
    let input = std::fs::read_to_string("input/day1.txt").unwrap();
    let res = part2(&input);
    println!("{res}");
}

#[test]
fn test() {
    assert_eq!(part2("L100"), 1);
    assert_eq!(part2("L100\nR100\nL100"), 3);
    assert_eq!(part2("L50\nL50"), 1);
    assert_eq!(part2("L3"), 0);
    assert_eq!(part2("R3"), 0);
    assert_eq!(part2("L3\nL2\nL4"), 0);
    assert_eq!(part2("L0"), 0);
    assert_eq!(part2("R99"), 1);
}
