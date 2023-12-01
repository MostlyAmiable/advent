#![allow(unused)]

use std::collections::HashSet;

type Point = (isize, isize);

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let instructions = input();
    let mut input = Input::new(2);
    for i in instructions {
        input.step(i);
    }
    dbg!(input.trail.len());
}

fn part_two() {
    let instructions = input();
    let mut input = Input::new(10);
    for i in instructions {
        input.step(i);
    }
    dbg!(input.trail.len());
}

enum Instruction {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (l, r) = s.split_once(' ').unwrap();
        let x = r.parse().unwrap();
        match l {
            "U" => Self::Up(x),
            "D" => Self::Down(x),
            "L" => Self::Left(x),
            "R" => Self::Right(x),
            _ => panic!("Invalid char: {l}"),
        }
    }
}

struct Input {
    trail: HashSet<(Point)>,
    // head: Point,
    // tail: Point,
    knots: Vec<Point>,
}

impl Input {
    fn new(n: usize) -> Self {
        Self { trail: HashSet::new(), knots: vec![(0, 0); n] }
    }

    fn step(&mut self, i: Instruction) {
        use Instruction::*;
        match i {
            Up(x) => {
                for _ in 0..x {
                    self.knots[0].1 += 1;
                    self.follow_all();
                }
            }
            Down(x) => {
                for _ in 0..x {
                    self.knots[0].1 -= 1;
                    self.follow_all();
                }
            }
            Left(x) => {
                for _ in 0..x {
                    self.knots[0].0 -= 1;
                    self.follow_all();
                }
            }
            Right(x) => {
                for _ in 0..x {
                    self.knots[0].0 += 1;
                    self.follow_all();
                }
            }
        }
    }

    fn follow_all(&mut self) {
        for i in 0..self.knots.len() - 1 {
            if !Self::follow(self.knots[i], &mut self.knots[i + 1]) {
                break;
            }
        }
        self.trail.insert(*self.knots.last().unwrap());
    }

    fn follow(a: Point, b: &mut Point) -> bool {
        // if a.0.abs_diff(b.0) > 2 || a.1.abs_diff(b.1) > 2 {
        //     println!("{:?}-{:?}", &a, b);
        // }
        if a.0.abs_diff(b.0) > 1 || a.1.abs_diff(b.1) > 1 {
            b.0 += (a.0 - b.0).signum();
            b.1 += (a.1 - b.1).signum();
            true
        } else {
            false
        }
    }
}

fn input() -> Vec<Instruction> {
    include_str!("./input.txt").lines().map(Instruction::parse).collect()
}

fn _test() -> Vec<Instruction> {
    include_str!("./test.txt").lines().map(Instruction::parse).collect()
}

fn _test2() -> Vec<Instruction> {
    include_str!("./test2.txt").lines().map(Instruction::parse).collect()
}
