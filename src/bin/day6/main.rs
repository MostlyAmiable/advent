#![allow(unused)]

use std::collections::HashSet;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    dbg!(Input::new().run(input(), 4));
    // include_str!("./test.txt").lines().for_each(|s| {
    //     dbg!(Input::new().run(s.as_bytes().to_owned()));
    // });
}

fn part_two() {
    dbg!(Input::new().run(input(), 14));
}

struct Input {
    buffer: Vec<u8>,
}

impl Input {
    fn run(&mut self, input: Vec<u8>, n: usize) -> usize {
        for (i, &c) in input.iter().enumerate() {
            if let Some(idx) = self.insert(c) {
                if idx == 3 {
                    self.buffer.clear();
                } else {
                    self.buffer = Vec::from(&self.buffer[idx + 1..]);
                }
                self.buffer.push(c);
                // println!("Clearing: {i}-{c} ({} remaining)", self.buffer.len());
            } else if self.buffer.len() == n {
                return i + 1;
            }
        }
        0
    }

    fn insert(&mut self, c: u8) -> Option<usize> {
        for (i, &v) in self.buffer.iter().enumerate() {
            if v == c {
                return Some(i);
            }
        }
        self.buffer.push(c);
        return None;
    }

    fn new() -> Self {
        let buffer = Vec::new();
        Self { buffer }
    }
}

fn input() -> Vec<u8> {
    include_str!("./input.txt").as_bytes().to_owned()
}

fn _test() -> Vec<u8> {
    include_str!("./test.txt").as_bytes().to_owned()
}
