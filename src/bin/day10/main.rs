#![allow(unused)]

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let instructions = input();
    dbg!(Input::new(instructions.into_iter()).run1());
}

fn part_two() {
    let instructions = input();
    Input::new(instructions.into_iter()).run2();
}

enum Instruction {
    Add(isize),
    Noop,
}

impl Instruction {
    fn parse(s: &str) -> Self {
        if let Some((l, r)) = s.split_once(' ') {
            Self::Add(r.parse().unwrap())
        } else {
            Self::Noop
        }
    }

    fn cycles(&self) -> usize {
        match self {
            Self::Add(_) => 2,
            Self::Noop => 1,
        }
    }

    fn value(&self) -> isize {
        match self {
            Self::Add(i) => *i,
            Self::Noop => 0,
        }
    }
}

struct Input<I> {
    instructions: I,
    register: isize,
    total: isize,
    cycle: usize,
}

impl<I: Iterator<Item = Instruction>> Input<I> {
    fn new(instructions: I) -> Self {
        Self { instructions, register: 1, total: 0, cycle: 0 }
    }

    fn run1(&mut self) -> isize {
        while let Some(i) = self.instructions.next() {
            for c in 1..=i.cycles() {
                let cycle = self.cycle + c;
                if (cycle + 20) % 40 == 0 {
                    // dbg!(cycle, self.register);
                    self.total += self.register * cycle as isize;
                }
            }
            self.cycle += i.cycles();
            self.register += i.value();
        }
        self.total
    }

    fn run2(&mut self) {
        // self.cycle += 1;
        let mut output = String::new();
        while let Some(i) = self.instructions.next() {
            for c in 1..=i.cycles() {
                // dbg!(cycle, self.register);
                if (self.cycle % 40).abs_diff(self.register as usize) <= 1 {
                    output.push('#');
                } else {
                    output.push('.');
                }
                if self.cycle % 40 == 39 {
                    output.push('\n');
                }
                self.cycle += 1;
            }
            self.register += i.value();
        }
        println!("{output}");
    }
}

fn input() -> Vec<Instruction> {
    include_str!("./input.txt").lines().map(Instruction::parse).collect()
}

fn _test() -> Vec<Instruction> {
    include_str!("./test.txt").lines().map(Instruction::parse).collect()
}
