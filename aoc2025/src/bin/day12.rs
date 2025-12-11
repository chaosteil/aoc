use itertools::Itertools;

// const TEST_INPUT: &str = include_str!("../../inputs/day12/test.txt");
const INPUT: &str = include_str!("../../inputs/day12/input.txt");

fn main() {
    // assert_eq!(count_regions(&parse_input(TEST_INPUT)), 2);
    println!("{:?}", count_regions(&parse_input(INPUT)));
}

#[derive(PartialEq, Eq)]
enum PresentData {
    Empty,
    Occupied,
}

struct Present {
    shape: Vec<Vec<PresentData>>,
}

impl Present {
    fn cells(&self) -> usize {
        self.shape
            .iter()
            .map(|v| v.iter().filter(|p| **p == PresentData::Occupied).count())
            .sum()
    }
}

struct Region {
    width: usize,
    height: usize,
    presents: Vec<usize>,
}

struct Data {
    presents: Vec<Present>,
    regions: Vec<Region>,
}

fn parse_input(input: &str) -> Data {
    let mut presents = Vec::new();
    for present in &input.lines().take(30).chunks(5) {
        let mut p = Vec::new();
        for chunk in present.skip(1).take(3) {
            p.push(
                chunk
                    .chars()
                    .map(|c| match c {
                        '#' => PresentData::Occupied,
                        '.' => PresentData::Empty,
                        _ => unreachable!(),
                    })
                    .collect(),
            )
        }
        presents.push(Present { shape: p });
    }
    let regions = input
        .lines()
        .skip(30)
        .map(|line| {
            let (size, presents) = line.split_once(':').unwrap();
            let (x, y) = size.split_once('x').unwrap();
            Region {
                width: x.parse().unwrap(),
                height: y.parse().unwrap(),
                presents: presents
                    .trim_ascii()
                    .split(' ')
                    .map(|i| i.parse().unwrap())
                    .collect(),
            }
        })
        .collect();
    Data { presents, regions }
}

fn count_regions(data: &Data) -> usize {
    let mut valid = 0;
    for region in &data.regions {
        let size = region.width * region.height;
        let mut total = 0;
        for (p, r) in data.presents.iter().zip(region.presents.iter()) {
            total += p.cells() * r
        }
        if size >= total {
            valid += 1;
        }
    }
    valid
}
