use std::collections::{HashMap, HashSet};

const TEST_INPUT: &str = include_str!("../../inputs/day07/test.txt");
const INPUT: &str = include_str!("../../inputs/day07/input.txt");

fn main() {
    assert_eq!(count_splits(&parse_input(TEST_INPUT)), 21);
    println!("{:?}", count_splits(&parse_input(INPUT)));

    assert_eq!(count_timelines(&parse_input(TEST_INPUT)), 40);
    println!("{:?}", count_timelines(&parse_input(INPUT)));
}

#[derive(PartialEq, Debug, Eq, Copy, Clone)]
enum Manifold {
    Empty,
    Start,
    Splitter,
    Beam,
}

#[derive(Debug, Clone)]
struct Data {
    data: Vec<Vec<Manifold>>,
}

fn parse_input(input: &str) -> Data {
    let data = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Manifold::Empty,
                    'S' => Manifold::Start,
                    '^' => Manifold::Splitter,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect();
    Data { data }
}

fn count_splits(data: &Data) -> usize {
    let mut data = data.data.clone();
    let mut splits = 0;
    for y in 0..data.len() {
        for x in 0..data[y].len() {
            if y == 0 && data[y][x] != Manifold::Start {
                continue;
            }
            match data[y][x] {
                Manifold::Start => {
                    data[y + 1][x] = Manifold::Beam;
                }
                Manifold::Empty => {
                    if data[y - 1][x] == Manifold::Beam {
                        data[y][x] = Manifold::Beam
                    }
                }
                Manifold::Splitter => {
                    if data[y - 1][x] == Manifold::Beam {
                        splits += 1;
                        data[y][x - 1] = Manifold::Beam;
                        data[y][x + 1] = Manifold::Beam;
                    }
                }
                _ => {}
            }
        }
    }
    splits
}

fn count_timelines(data: &Data) -> usize {
    let mut results = HashMap::new();
    count_timelines_recursive(data, (0, 0), &mut results) + 1
}

fn count_timelines_recursive(
    d: &Data,
    start: (usize, usize),
    results: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if let Some(value) = results.get(&start) {
        return *value;
    }
    let data = &d.data;
    let mut beams = HashSet::new();
    let mut timelines = 0;
    if start != (0, 0) {
        beams.insert(start);
    }
    for (y, line) in data.iter().enumerate().skip(start.1) {
        for (x, m) in line.iter().enumerate() {
            if y == 0 && *m != Manifold::Start {
                continue;
            }
            match m {
                Manifold::Start => {
                    beams.insert((x, y + 1));
                }
                Manifold::Empty => {
                    if beams.contains(&(x, y - 1)) {
                        beams.insert((x, y));
                    }
                }
                Manifold::Splitter => {
                    if beams.contains(&(x, y - 1)) {
                        timelines += 1;
                        timelines += count_timelines_recursive(d, (x - 1, y), results);
                        timelines += count_timelines_recursive(d, (x + 1, y), results);
                    }
                }
                _ => {}
            }
        }
    }
    results.insert(start, timelines);
    timelines
}
