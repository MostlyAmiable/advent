#![allow(unused)]

use ndarray::Array2;

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
}

fn part_two() {}

struct Input {
    trees: Array2<u8>,
}
impl Input {
    fn parse(s: &str, n: usize) -> Self {
        let trees = Array2::from_shape_vec((n, n), s.lines().flat_map(|l| l.as_bytes().iter().map(|c| c - b'0')).collect()).unwrap();
        Self { trees }
    }
}

fn input() -> Input {
    Input::parse(include_str!("./input.txt"), 5)
}

fn _test() -> Input {
    Input::parse(include_str!("./test.txt"), 99)
}
