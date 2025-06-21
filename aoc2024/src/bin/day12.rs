use std::collections::HashMap;
use std::fs;
use std::time::Instant;

#[derive(Default)]
struct Region {
    area: u32,
    perimeter: u32,
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

    fn get_fence_price(&self) -> u64 {
        let mut price: u64 = 0;
        for region in self.regions.values() {
            price += (region.area as u64) * (region.perimeter as u64);
        }
        price
    }
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

pub fn part2(input: &str) -> u64 {
    let file_content = fs::read_to_string(input).unwrap();
    let _lines = file_content.split("\n");

    return 0;
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
