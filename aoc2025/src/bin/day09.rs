const TEST_INPUT: &str = include_str!("../../inputs/day09/test.txt");
const INPUT: &str = include_str!("../../inputs/day09/input.txt");

fn main() {
    assert_eq!(largest_area(&parse_input(TEST_INPUT)), 50);
    println!("{:?}", largest_area(&parse_input(INPUT)));

    assert_eq!(largest_area_red_green(&parse_input(TEST_INPUT)), 24);
    println!("{:?}", largest_area_red_green(&parse_input(INPUT)));
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn area(&self, other: &Self) -> usize {
        (self.x.max(other.x) - self.x.min(other.x) + 1)
            * (self.y.max(other.y) - self.y.min(other.y) + 1)
    }

    fn top_left(&self, other: &Self) -> Position {
        Position {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
    fn bottom_right(&self, other: &Self) -> Position {
        Position {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }
}

struct Data {
    positions: Vec<Position>,
}

fn parse_input(input: &str) -> Data {
    Data {
        positions: input
            .lines()
            .map(|l| {
                let l = l.split_once(',').unwrap();
                Position {
                    x: l.0.parse().unwrap(),
                    y: l.1.parse().unwrap(),
                }
            })
            .collect(),
    }
}

fn largest_area(data: &Data) -> usize {
    let mut biggest = 0;
    for (pos, a) in data.positions.iter().enumerate() {
        for b in data.positions.iter().skip(pos) {
            biggest = biggest.max(a.area(b))
        }
    }
    biggest
}

#[derive(Debug, Copy, Clone)]
struct Edge {
    from: Position,
    to: Position,
}

impl Edge {
    fn outside(&self, a: &Position, b: &Position) -> bool {
        let top_left = a.top_left(b);
        let bottom_right = a.bottom_right(b);
        !((self.from.x <= top_left.x && self.to.x <= top_left.x)
            || (self.from.y <= top_left.y && self.to.y <= top_left.y)
            || (self.from.x + 1 >= bottom_right.x && self.to.x + 1 >= bottom_right.x)
            || (self.from.y + 1 >= bottom_right.y && self.to.y + 1 >= bottom_right.y))
    }
}

fn largest_area_red_green(data: &Data) -> usize {
    let mut edges = Vec::new();
    for i in 0..data.positions.len() {
        let a = data.positions[i];
        let b = data.positions[(i + 1) % data.positions.len()];
        edges.push(Edge {
            from: a.top_left(&b),
            to: a.bottom_right(&b),
        })
    }
    let mut biggest = 0;
    for (pos_a, a) in data.positions.iter().enumerate() {
        'check: for (pos_b, b) in data.positions.iter().enumerate() {
            if pos_a == pos_b {
                continue;
            }
            for edge in &edges {
                if edge.outside(a, b) {
                    continue 'check;
                }
            }
            biggest = biggest.max(a.area(b));
        }
    }
    biggest
}
