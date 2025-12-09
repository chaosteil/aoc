const TEST_INPUT: &str = include_str!("../../inputs/day10/test.txt");
const INPUT: &str = include_str!("../../inputs/day10/input.txt");

fn main() {
    assert_eq!(fewest_button_presses(&parse_input(TEST_INPUT)), 7);
    println!("{:?}", fewest_button_presses(&parse_input(INPUT)));

    assert_eq!(fewest_joltage_presses(&parse_input(TEST_INPUT)), 33);
    println!("{:?}", fewest_joltage_presses(&parse_input(INPUT)));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Light {
    On,
    Off,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lights(Vec<Light>);

impl Lights {
    fn apply(&mut self, wires: &[usize]) {
        for w in wires {
            self.toggle(*w);
        }
    }

    fn undo(&mut self, wires: &[usize]) {
        self.apply(wires);
    }

    fn toggle(&mut self, num: usize) {
        self.0[num] = match self.0[num] {
            Light::On => Light::Off,
            Light::Off => Light::On,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Joltage(Vec<usize>);

#[derive(Debug)]
struct Machine {
    lights: Lights,
    wires: Vec<Vec<usize>>,
    joltage: Joltage,
}

#[derive(Debug)]
struct Data {
    machines: Vec<Machine>,
}

fn parse_input(input: &str) -> Data {
    Data {
        machines: input
            .lines()
            .map(|line| {
                let items = line.split(' ').collect::<Vec<_>>();
                Machine {
                    lights: Lights(
                        items
                            .first()
                            .unwrap()
                            .chars()
                            .skip(1)
                            .take_while(|c| *c != ']')
                            .map(|c| match c {
                                '.' => Light::Off,
                                '#' => Light::On,
                                _ => unreachable!(),
                            })
                            .collect(),
                    ),
                    wires: items[1..(items.len() - 1)]
                        .iter()
                        .map(|&item| {
                            item.strip_prefix('(')
                                .unwrap()
                                .strip_suffix(')')
                                .unwrap()
                                .split(',')
                                .map(|c| c.parse::<usize>().unwrap())
                                .collect()
                        })
                        .collect(),
                    joltage: Joltage(
                        items
                            .last()
                            .unwrap()
                            .strip_prefix('{')
                            .unwrap()
                            .strip_suffix('}')
                            .unwrap()
                            .split(',')
                            .map(|c| c.parse().unwrap())
                            .collect(),
                    ),
                }
            })
            .collect(),
    }
}

fn find_button_presses(
    result: &Lights,
    state: &mut Lights,
    wires: &[Vec<usize>],
    cur_depth: usize,
    max_depth: usize,
) -> bool {
    if state == result {
        return true;
    }
    if cur_depth == max_depth {
        return false;
    }
    for wire in wires {
        state.apply(wire);
        if find_button_presses(result, state, wires, cur_depth + 1, max_depth) {
            return true;
        }
        state.undo(wire);
    }
    false
}

fn fewest_button_presses(data: &Data) -> usize {
    let mut total = 0;
    'machine: for machine in &data.machines {
        let mut max_depth = 0;
        loop {
            let mut initial = Lights(vec![Light::Off; machine.lights.0.len()]);
            if find_button_presses(&machine.lights, &mut initial, &machine.wires, 0, max_depth) {
                total += max_depth;
                continue 'machine;
            }
            max_depth += 1;
        }
    }
    total
}

fn fewest_joltage_presses(data: &Data) -> usize {
    let mut total = 0;
    for machine in &data.machines {
        // Heavily leaned on a solution in another language I found online, first time I deal with
        // an ILP solver
        use good_lp::*;
        let mut vars = good_lp::variables!();
        // Get a list of variables bounded to the bottom to 0, one for each set of wires
        let presses: Vec<_> = machine
            .wires
            .iter()
            .map(|_| vars.add(good_lp::variable().min(0).integer()))
            .collect();
        // Minimize the sum of the presses
        let mut problem =
            good_lp::lp_solve(vars.minimise(presses.iter().sum::<good_lp::Expression>()));
        let mut expressions = vec![good_lp::Expression::from(0); machine.joltage.0.len()];
        for (i, wires) in machine.wires.iter().enumerate() {
            for wire in wires {
                // We add a button press to each variable
                expressions[*wire] += presses[i];
            }
        }
        // Expressions are now v0 + v1, v0, v2 + v1, etc.
        for (i, j) in machine.joltage.0.iter().enumerate() {
            // Final state of each expression should be this.
            problem.add_constraint(expressions[i].clone().eq(*j as u32));
        }
        // Automatically find the solution
        let solution = problem.solve().unwrap();
        for v in presses {
            total += solution.value(v) as usize;
        }
    }
    total
}
