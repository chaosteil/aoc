use std::collections::HashSet;

const TEST_INPUT: &str = include_str!("../../inputs/day04/test.txt");
const INPUT: &str = include_str!("../../inputs/day04/input.txt");

fn main() {
    assert_eq!(find_paper(&parse_map(TEST_INPUT)).len(), 13);
    println!("{:?}", find_paper(&parse_map(INPUT)).len());

    assert_eq!(find_all_paper(&parse_map(TEST_INPUT)), 43);
    println!("{:?}", find_all_paper(&parse_map(INPUT)));
}

fn find_paper(map: &Map) -> Vec<(usize, usize)> {
    let mut papers = Vec::new();
    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            if map.grid[y][x] != Items::Paper {
                continue;
            }
            let surrounding = map.surrounding_papers(x, y);
            if surrounding.len() < 4 {
                papers.push((x, y));
            }
        }
    }
    papers
}

fn find_all_paper(map: &Map) -> usize {
    let mut set = HashSet::<(usize, usize)>::new();
    let mut map = map.clone();
    loop {
        let papers = find_paper(&map);
        if papers.is_empty() {
            break;
        }
        set.extend(&papers);
        for (x, y) in papers {
            map.grid[y][x] = Items::None;
        }
    }
    set.len()
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Items {
    None,
    Paper,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<Items>>,
}

impl Map {
    fn surrounding_papers(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut papers = Vec::with_capacity(8);
        let (x, y) = (x as isize, y as isize);
        for dy in -1isize..=1 {
            for dx in -1isize..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                if x + dx < 0
                    || y + dy < 0
                    || y + dy >= self.grid.len() as isize
                    || x + dx >= self.grid[y as usize].len() as isize
                {
                    continue;
                }

                let (x, y) = (((x + dx) as usize), ((y + dy) as usize));
                if self.grid[y][x] == Items::Paper {
                    papers.push((x, y));
                }
            }
        }
        papers
    }
}

fn parse_map(input: &str) -> Map {
    Map {
        grid: input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Items::None,
                        '@' => Items::Paper,
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
    }
}
