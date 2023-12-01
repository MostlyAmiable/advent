#![allow(unused)]

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let total: usize = input().map(|(a, b)| Strat::parse(a).round(&Strat::parse(b))).sum();
    dbg!(total);
}

fn part_two() {
    let total: usize = input().map(|(a, b)| Strat::parse(a).round2(&Outcome::parse(b))).sum();
    dbg!(total);
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Strat {
    Rock,
    Paper,
    Scissors,
}

impl Strat {
    fn parse(c: u8) -> Self {
        match c {
            b'A' | b'X' => Self::Rock,
            b'B' | b'Y' => Self::Paper,
            b'C' | b'Z' => Self::Scissors,
            _ => panic!("Invalid char: {c}"),
        }
    }

    fn round(&self, me: &Self) -> usize {
        self.evaluate(me) + me.value()
    }

    fn evaluate(&self, me: &Self) -> usize {
        match (self, me) {
            (Strat::Rock, Strat::Scissors) => 0,
            (Strat::Rock, Strat::Paper) => 6,
            (Strat::Paper, Strat::Rock) => 0,
            (Strat::Paper, Strat::Scissors) => 6,
            (Strat::Scissors, Strat::Paper) => 0,
            (Strat::Scissors, Strat::Rock) => 6,
            _ => 3,
        }
    }

    fn round2(&self, out: &Outcome) -> usize {
        self.plan(out).value() + out.value()
    }

    fn plan(&self, out: &Outcome) -> Self {
        match (self, out) {
            (Strat::Rock, Outcome::Loss) => Self::Scissors,
            (Strat::Rock, Outcome::Win) => Self::Paper,
            (Strat::Paper, Outcome::Loss) => Self::Rock,
            (Strat::Paper, Outcome::Win) => Self::Scissors,
            (Strat::Scissors, Outcome::Loss) => Self::Paper,
            (Strat::Scissors, Outcome::Win) => Self::Rock,
            _ => *self,
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Tie,
    Win,
}

impl Outcome {
    fn parse(c: u8) -> Self {
        match c {
            b'X' => Self::Loss,
            b'Y' => Self::Tie,
            b'Z' => Self::Win,
            _ => panic!("Invalid char: {c}"),
        }
    }

    fn value(&self) -> usize {
        match self {
            Self::Loss => 0,
            Self::Tie => 3,
            Self::Win => 6,
        }
    }
}

fn parse(s: &'static str) -> impl Iterator<Item = (u8, u8)> {
    s.lines().map(|s| {
        let b = s.as_bytes();
        (b[0], b[2])
    })
}

fn input() -> impl Iterator<Item = (u8, u8)> {
    parse(include_str!("./input.txt"))
}

fn test() -> impl Iterator<Item = (u8, u8)> {
    parse(include_str!("./test.txt"))
}
