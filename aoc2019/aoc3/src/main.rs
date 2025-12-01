use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fs::File,
    io::{self, BufRead},
};

#[derive(Debug)]
struct Op {
    dir: char,
    i: isize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input/input")?;
    let lines: Vec<Vec<Op>> = io::BufReader::new(file)
        .lines()
        .filter_map(Result::ok)
        .map(|x| {
            x.split(',')
                .map(|v| Op {
                    dir: v.as_bytes()[0] as char,
                    i: v[1..].parse::<isize>().unwrap(),
                })
                .collect()
        })
        .collect();
    println!("{:?}", part_one(&lines));
    println!("{:?}", part_two(&lines));
    Ok(())
}

fn part_one(input: &[Vec<Op>]) -> isize {
    let hm = traverse(input);
    hm.iter()
        .filter(|(k, v)| k.0 != 0 && k.1 != 0 && v[0] != 0 && v[1] != 0)
        .map(|((x, y), _)| x.abs() + y.abs())
        .min()
        .unwrap()
}
fn part_two(input: &[Vec<Op>]) -> isize {
    let hm = traverse(input);
    hm.iter()
        .filter(|(k, v)| k.0 != 0 && k.1 != 0 && v[0] != 0 && v[1] != 0)
        .map(|(_, v)| v[0] + v[1])
        .min()
        .unwrap()
}

fn traverse(input: &[Vec<Op>]) -> HashMap<(isize, isize), [isize; 2]> {
    let mut hm = HashMap::<_, [isize; 2]>::new();
    for (i, line) in input.iter().enumerate() {
        let (mut x, mut y, mut step) = (0, 0, 0);
        for op in line {
            match op.dir {
                'U' => {
                    for newy in y + 1..=(y + op.i) {
                        let e = hm.entry((x, newy)).or_default();
                        step += 1;
                        e[i] = step;
                    }
                    y += op.i;
                }
                'D' => {
                    for newy in (y - op.i..y).rev() {
                        let e = hm.entry((x, newy)).or_default();
                        step += 1;
                        e[i] = step;
                    }
                    y -= op.i;
                }
                'L' => {
                    for newx in (x - op.i..x).rev() {
                        let e = hm.entry((newx, y)).or_default();
                        step += 1;
                        e[i] = step;
                    }
                    x -= op.i;
                }
                'R' => {
                    for newx in x + 1..=(x + op.i) {
                        let e = hm.entry((newx, y)).or_default();
                        step += 1;
                        e[i] = step;
                    }
                    x += op.i;
                }
                _ => unreachable!(),
            }
        }
    }
    hm
}
