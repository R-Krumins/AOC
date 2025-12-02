use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Default)]
struct Region {
    area: u32,
    perimeter: u32,
    corner_count: u32,
}

struct Plot {
    ch: char,
    visited: bool,
}

struct Garden {
    width: usize,
    height: usize,
    grid: Vec<Vec<Plot>>,
    regions: HashMap<usize, Region>,
}

impl Garden {
    fn new(input: &str) -> Self {
        let grid: Vec<Vec<Plot>> = input
            .trim()
            .lines()
            .map(|line| line.chars().map(|ch| Plot { ch, visited: false }).collect())
            .collect();

        Garden {
            width: grid.len(),
            height: grid[0].len(),
            grid,
            regions: HashMap::new(),
        }
    }

    fn within_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    fn was_visited(&self, x: usize, y: usize) -> bool {
        self.grid[y][x].visited
    }

    fn mark_visited(&mut self, x: usize, y: usize) {
        self.grid[y][x].visited = true;
    }

    fn get_ch(&self, x: usize, y: usize) -> char {
        self.grid[y][x].ch
    }

    fn increment_perimter(&mut self, region_id: usize) {
        self.regions.entry(region_id).or_default().perimeter += 1;
    }

    fn increment_area(&mut self, region_id: usize) {
        self.regions.entry(region_id).or_default().area += 1;
    }

    fn increment_corner_count(&mut self, region_id: usize) {
        self.regions.entry(region_id).or_default().corner_count += 1;
    }

    fn get_fence_price(&self) -> u64 {
        let mut price: u64 = 0;
        for region in self.regions.values() {
            price += (region.area as u64) * (region.perimeter as u64);
        }
        price
    }

    fn get_fence_price_part2(&self) -> u64 {
        let mut price: u64 = 0;
        for region in self.regions.values() {
            price += (region.area as u64) * (region.corner_count as u64);
        }
        price
    }

    fn is_corner(&self, x: usize, y: usize, target: char) -> bool {
        !(self.within_bounds(x, y) && self.get_ch(x, y) == target)
    }
    fn identify_corner(&self, x: usize, y: usize) -> Corner {
        let target = self.get_ch(x, y);
        println!("{target}");
        let bottom = self.is_corner(x, y + 1, target);
        let left = self.is_corner(x + 1, y, target);
        let top = if y > 0 {
            self.is_corner(x, y - 1, target)
        } else {
            true
        };
        let right = if x > 0 {
            self.is_corner(x - 1, y, target)
        } else {
            true
        };

        println!("x: {x}, y: {y}");
        println!("corners: {top}, {left}, {bottom}, {right}");

        Corner::new(top, left, bottom, right)
    }
}

enum Corner {
    T,
    B,
    L,
    R,
    TB,
    LR,
    TL,
    TR,
    BL,
    BR,
    LTR,
    LBR,
    BLT,
    BRT,
    TLBR,
    NotCorner,
}

impl Corner {
    fn new(top: bool, left: bool, bottom: bool, right: bool) -> Corner {
        use Corner::*;
        match (top, left, bottom, right) {
            (true, false, false, false) => T,
            (false, true, false, false) => L,
            (false, false, true, false) => B,
            (false, false, false, true) => R,
            (true, false, true, false) => TB,
            (false, true, false, true) => LR,
            (true, true, false, false) => TL,
            (true, false, false, true) => TR,
            (false, true, true, false) => BL,
            (false, false, true, true) => BR,
            (true, true, false, true) => LTR,
            (false, true, true, true) => LBR,
            (true, true, true, false) => BLT,
            (true, false, true, true) => BRT,
            (true, true, true, true) => TLBR,
            (false, false, false, false) => NotCorner,
        }
    }

    fn next_direction(&self, current_dir: &Direction) -> Option<Direction> {
        use Corner::*;
        match self {
            T | TR | BRT => Some(Direction::LEFT),
            L | TL | LTR => Some(Direction::DOWN),
            B | BL | BLT => Some(Direction::RIGHT),
            R | BR | LBR => Some(Direction::UP),
            TB => match current_dir {
                Direction::LEFT => Some(Direction::LEFT),
                Direction::RIGHT => Some(Direction::RIGHT),
                _ => panic!(),
            },
            LR => match current_dir {
                Direction::UP => Some(Direction::UP),
                Direction::DOWN => Some(Direction::DOWN),
                _ => panic!(),
            },
            TLBR => None,
            NotCorner => match current_dir {
                Direction::UP => Some(Direction::RIGHT),
                Direction::RIGHT => Some(Direction::DOWN),
                _ => panic!(),
            },
        }
    }
}

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn walk(&self, x: usize, y: usize) -> (usize, usize) {
        use Direction::*;
        match self {
            UP => (x, y - 1),
            DOWN => (x, y + 1),
            LEFT => (x + 1, y),
            RIGHT => (x - 1, y),
        }
    }
}
fn search_region(x: usize, y: usize, region_id: usize, target: char, garden: &mut Garden) {
    if !garden.within_bounds(x, y) {
        garden.increment_perimter(region_id);
        return;
    }

    if garden.get_ch(x, y) != target {
        garden.increment_perimter(region_id);
        return;
    }

    if garden.was_visited(x, y) {
        return;
    }

    // found new plot in region
    // =============================
    garden.mark_visited(x, y);
    garden.increment_area(region_id);
    // ==============================

    // search recursivly
    search_region(x + 1, y, region_id, target, garden);
    search_region(x, y + 1, region_id, target, garden);

    if x > 0 {
        search_region(x - 1, y, region_id, target, garden);
    } else {
        garden.increment_perimter(region_id);
    }

    if y > 0 {
        search_region(x, y - 1, region_id, target, garden);
    } else {
        garden.increment_perimter(region_id);
    }
}

fn edge_walk(
    src_x: usize,
    src_y: usize,
    curr_x: usize,
    curr_y: usize,
    curr_dir: Direction,
    region_id: usize,
    garden: &mut Garden,
    firt_walk: bool,
) {
    if !firt_walk && src_x == curr_x && src_y == curr_y {
        return;
    }

    let next_dir = garden
        .identify_corner(curr_x, curr_y)
        .next_direction(&curr_dir);

    let Some(next_dir) = next_dir else {
        garden.increment_corner_count(region_id);
        return;
    };

    if curr_dir != next_dir {
        garden.increment_corner_count(region_id);
    }

    let (next_x, next_y) = next_dir.walk(curr_x, curr_y);

    edge_walk(
        src_x, src_y, next_x, next_y, next_dir, region_id, garden, false,
    );
}
pub fn part1(input: &str) -> u64 {
    let input = fs::read_to_string(input).unwrap();

    let mut garden = Garden::new(&input);

    let mut region_id: usize = 0;
    for y in 0..garden.height {
        for x in 0..garden.width {
            if garden.was_visited(x, y) {
                continue;
            }

            let char_to_search = garden.get_ch(x, y);
            search_region(x, y, region_id, char_to_search, &mut garden);
            region_id += 1;
        }
    }

    return garden.get_fence_price();
}

pub fn part2(input: &str) -> u64 {
    let input = fs::read_to_string(input).unwrap();

    let mut garden = Garden::new(&input);

    let mut region_id: usize = 0;
    for y in 0..garden.height {
        for x in 0..garden.width {
            if garden.was_visited(x, y) {
                continue;
            }

            let char_to_search = garden.get_ch(x, y);
            search_region(x, y, region_id, char_to_search, &mut garden);
            edge_walk(x, y, x, y, Direction::LEFT, region_id, &mut garden, true);
            region_id += 1;
        }
    }

    return garden.get_fence_price_part2();
}

fn main() {
    let input = "src/inputs/day12.txt";

    let (do_part1, do_part2) = match std::env::args().nth(1).as_deref() {
        Some("1") => (true, false),
        Some("2") => (false, true),
        Some("12") | Some("21") | None => (true, true),
        _ => panic!("Invalid arg. Pass in 1, 2 or leave empty for both."),
    };

    if do_part1 {
        let start = Instant::now();
        let res = part1(input);
        let duration = start.elapsed();
        println!("{res}");
        println!("{duration:?}");
    }

    if do_part2 {
        let start = Instant::now();
        let res = part2(input);
        let duration = start.elapsed();
        println!("{res}");
        println!("{duration:?}");
    }
}
