use std::{cell::RefCell, collections::HashMap, rc::Rc};

fn main() -> tools::Result<()> {
    let input = process_input(tools::read_input()?);
    tools::print_result(part_one(&input), part_two(&input));
    Ok(())
}

#[derive(Debug, Default)]
struct Node {
    size: usize,
    nodes: HashMap<String, Rc<RefCell<Node>>>,
}

impl Node {
    fn new(size: usize) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            size,
            ..Default::default()
        }))
    }
}

type Input = Rc<RefCell<Node>>;

fn process_input(input: Vec<String>) -> Input {
    let root = Node::new(0);
    let mut stack = Vec::new();
    let mut current = root.clone();
    for i in input {
        match i.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "cd", "/"] => {}
            ["$", "cd", ".."] => {
                current = stack.pop().unwrap();
            }
            ["$", "cd", dir] => {
                let n = current.borrow_mut().nodes.get(dir).unwrap().clone();
                stack.push(current);
                current = n;
            }
            ["$", "ls"] => {}
            ["dir", dir] => {
                current.borrow_mut().nodes.insert(
                    dir.to_owned(),
                    Rc::new(RefCell::new(Node {
                        ..Default::default()
                    })),
                );
            }
            [size, file] => {
                let size = size.parse::<usize>().unwrap();
                let mut current = current.borrow_mut();
                current.nodes.insert(file.to_owned(), Node::new(size));
                current.size += size;
                for s in stack.iter_mut() {
                    s.borrow_mut().size += size;
                }
            }
            _ => unreachable!(),
        }
    }
    root
}
fn part_one(input: &Input) -> usize {
    let mut size = 0;
    let mut eval = vec![input.clone()];
    while !eval.is_empty() {
        let node = eval.pop().unwrap();
        for (_, n) in node.borrow().nodes.iter() {
            eval.push(n.clone());
        }
        let n = node.borrow();
        if !n.nodes.is_empty() && n.size <= 100000 {
            size += n.size;
        }
    }
    size
}

fn part_two(input: &Input) -> usize {
    let needs = 30000000 - (70000000 - input.borrow().size);
    let mut sizes = vec![];
    let mut eval = vec![input.clone()];
    while !eval.is_empty() {
        let node = eval.pop().unwrap();
        let node = node.borrow();
        for (_, n) in node.nodes.iter() {
            eval.push(n.clone());
        }
        if !node.nodes.is_empty() {
            sizes.push(node.size);
        }
    }
    sizes.sort();
    *sizes.iter().find(|&f| *f >= needs).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn testrun() -> tools::Result<()> {
        let input = super::process_input(tools::read_testinput()?);
        tools::print_result(super::part_one(&input), super::part_two(&input));
        Ok(())
    }
}
