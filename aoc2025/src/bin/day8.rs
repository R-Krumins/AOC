#[derive(Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    fn parse(s: &str) -> Self {
        let mut s = s.split(",");
        Self {
            x: s.next().unwrap().parse().unwrap(),
            y: s.next().unwrap().parse().unwrap(),
            z: s.next().unwrap().parse().unwrap(),
        }
    }

    fn distance(&self, other: Self) -> f64 {
        let a = self.x - other.x;
        let b = self.y - other.y;
        let c = self.z - other.z;
        // lazy distance calc with no sqrt
        a * a + b * b + c * c
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Box {
    pos: Vec3,
    parent: Option<usize>,
    children_count: u64,
}

impl Box {
    fn new(pos: Vec3) -> Self {
        Self {
            pos,
            parent: None,
            children_count: 0,
        }
    }

    fn find_root(idx: usize, lst: &[Self]) -> usize {
        let parent = lst[idx].parent.unwrap();
        if parent == idx {
            idx
        } else {
            Self::find_root(parent, lst)
        }
    }

    #[inline(always)]
    fn in_circuit(&self) -> bool {
        self.parent.is_some()
    }
}

pub fn part1(input: &str, max_connection: usize) -> u64 {
    let mut boxes: Vec<Box> = input.lines().map(Vec3::parse).map(Box::new).collect();

    // compute distances
    let cap = boxes.len() * (boxes.len() - 1) / 2;
    let mut distances: Vec<(usize, usize, f64)> = Vec::with_capacity(cap);
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = boxes[i].pos.distance(boxes[j].pos);
            distances.push((i, j, d));
        }
    }

    // sort ascending
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for (a, b, _) in distances.into_iter().take(max_connection) {
        match (boxes[a].in_circuit(), boxes[b].in_circuit()) {
            (false, false) => {
                boxes[b].parent = Some(a);
                boxes[a].parent = Some(a);
                boxes[a].children_count += 1;
            }
            (true, false) => {
                let a_root = Box::find_root(a, &boxes);
                boxes[b].parent = Some(a_root);
                boxes[a_root].children_count += 1;
            }
            (false, true) => {
                let b_root = Box::find_root(b, &boxes);
                boxes[a].parent = Some(b_root);
                boxes[b_root].children_count += 1;
            }
            (true, true) => {
                let a_root = Box::find_root(a, &boxes);
                let b_root = Box::find_root(b, &boxes);
                if a_root != b_root {
                    boxes[b_root].parent = Some(a_root);
                    boxes[a_root].children_count += boxes[b_root].children_count + 1;
                }
            }
        }
    }

    // count
    let mut circuit_box_counts: Vec<u64> = Vec::new();
    for i in 0..boxes.len() {
        if boxes[i].parent == Some(i) {
            circuit_box_counts.push(boxes[i].children_count + 1);
        }
    }

    circuit_box_counts.sort_by(|a, b| b.cmp(a));

    circuit_box_counts
        .into_iter()
        .take(3)
        .fold(1, |acc, n| acc * n)
}

pub fn part2(input: &str) -> u64 {
    let mut boxes: Vec<Box> = input.lines().map(Vec3::parse).map(Box::new).collect();

    // compute distances
    let cap = boxes.len() * (boxes.len() - 1) / 2;
    let mut distances: Vec<(usize, usize, f64)> = Vec::with_capacity(cap);
    for i in 0..boxes.len() {
        for j in (i + 1)..boxes.len() {
            let d = boxes[i].pos.distance(boxes[j].pos);
            distances.push((i, j, d));
        }
    }

    // sort ascending
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for (a, b, _) in distances {
        match (boxes[a].in_circuit(), boxes[b].in_circuit()) {
            (false, false) => {
                boxes[b].parent = Some(a);
                boxes[a].parent = Some(a);
                boxes[a].children_count += 1;
            }
            (true, false) => {
                let a_root = Box::find_root(a, &boxes);
                boxes[b].parent = Some(a_root);
                boxes[a_root].children_count += 1;
                if boxes[a_root].children_count + 1 == boxes.len() as u64 {
                    return (boxes[a].pos.x * boxes[b].pos.x) as u64;
                }
            }
            (false, true) => {
                let b_root = Box::find_root(b, &boxes);
                boxes[a].parent = Some(b_root);
                boxes[b_root].children_count += 1;
                if boxes[b_root].children_count + 1 == boxes.len() as u64 {
                    return (boxes[a].pos.x * boxes[b].pos.x) as u64;
                }
            }
            (true, true) => {
                let a_root = Box::find_root(a, &boxes);
                let b_root = Box::find_root(b, &boxes);
                if a_root != b_root {
                    boxes[b_root].parent = Some(a_root);
                    boxes[a_root].children_count += boxes[b_root].children_count + 1;
                }
                if boxes[a_root].children_count + 1 == boxes.len() as u64 {
                    return (boxes[a].pos.x * boxes[b].pos.x) as u64;
                }
            }
        }
    }
    0
}

fn main() {
    let input = std::fs::read_to_string("input/day8.txt").unwrap();

    let (do_part1, do_part2) = match std::env::args().nth(1).as_deref() {
        Some("1") => (true, false),
        Some("2") => (false, true),
        Some("12") | Some("21") | None => (true, true),
        Some(_) => panic!("Invalid part arg. Expected 1, 2 or empty for both"),
    };

    if do_part1 {
        let now = std::time::Instant::now();
        let res = part1(&input, 1000);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT, 10), 40);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 25272);
    }

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;
}
