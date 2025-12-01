use std::ops::{Index, IndexMut};

pub struct Intcode {
    ip: usize,
    stopped: bool,
    program: Vec<usize>,
}

impl Index<usize> for Intcode {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.program[index]
    }
}

impl IndexMut<usize> for Intcode {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.program[index]
    }
}

impl Intcode {
    pub fn new(program: &[usize]) -> Self {
        Self {
            ip: 0,
            stopped: false,
            program: program.to_vec(),
        }
    }

    pub fn parse(input: &str) -> Vec<usize> {
        input
            .split(',')
            .map(|i| i.parse::<usize>().unwrap())
            .collect()
    }

    pub fn run(&mut self) {
        while !self.stopped {
            self.step()
        }
    }

    fn step(&mut self) {
        if self.stopped {
            return;
        }
        self.ip += match self.program[self.ip] {
            1 => {
                let value = self.program[self.program[self.ip + 1]]
                    + self.program[self.program[self.ip + 2]];
                let pos = self.program[self.ip + 3];
                self.program[pos] = value;
                4
            }
            2 => {
                let value = self.program[self.program[self.ip + 1]]
                    * self.program[self.program[self.ip + 2]];
                let pos = self.program[self.ip + 3];
                self.program[pos] = value;
                4
            }
            99 => {
                self.stopped = true;
                1
            }
            _ => panic!(),
        }
    }
}
