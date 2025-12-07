use std::collections::HashSet;

const TEST_INPUT: &str = include_str!("../../inputs/day08/test.txt");
const INPUT: &str = include_str!("../../inputs/day08/input.txt");

fn main() {
    assert_eq!(largest_circuits(&parse_input(TEST_INPUT), Some(10)), 40);
    println!("{:?}", largest_circuits(&parse_input(INPUT), Some(1000)));

    assert_eq!(largest_circuits(&parse_input(TEST_INPUT), None), 25272);
    println!("{:?}", largest_circuits(&parse_input(INPUT), None));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct JunctionBox {
    x: usize,
    y: usize,
    z: usize,
}

impl JunctionBox {
    fn sq_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

struct Data {
    positions: Vec<JunctionBox>,
}

fn parse_input(input: &str) -> Data {
    Data {
        positions: input
            .lines()
            .map(|l| {
                let mut pos = l.split(',').map(|i| i.parse().unwrap());
                JunctionBox {
                    x: pos.next().unwrap(),
                    y: pos.next().unwrap(),
                    z: pos.next().unwrap(),
                }
            })
            .collect(),
    }
}

fn largest_circuits(data: &Data, limit: Option<usize>) -> usize {
    let mut circuits = Vec::new();
    for pos in &data.positions {
        let mut circuit = HashSet::new();
        circuit.insert(pos);
        circuits.push(circuit);
    }

    let mut positions = Vec::new();
    for (i, a) in data.positions.iter().enumerate() {
        for b in data.positions.iter().skip(i + 1) {
            positions.push((a, b, a.sq_distance(b)));
        }
    }
    positions.sort_by_key(|p| p.2);
    let positions = match limit {
        Some(limit) => positions.into_iter().take(limit).collect(),
        None => positions,
    };
    for (j1, j2, _) in positions {
        let (mut c1, mut c2) = (None, None);
        for (i, c) in circuits.iter().enumerate() {
            if c1.is_none() && c.contains(j1) {
                c1 = Some(i)
            }
            if c2.is_none() && c.contains(j2) {
                c2 = Some(i)
            }
            if c1.is_some() && c2.is_some() {
                break;
            }
        }
        let (c1, c2) = (c1.unwrap(), c2.unwrap());
        if c1 == c2 {
            continue;
        }
        let data = circuits[c2].clone();
        circuits[c1].extend(data);
        circuits.remove(c2);

        if limit.is_none() && circuits.len() == 1 {
            return j1.x * j2.x;
        }
    }

    circuits.sort_by_key(|c| c.len());
    circuits.iter().rev().take(3).map(|c| c.len()).product()
}
