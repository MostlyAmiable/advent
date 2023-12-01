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
    monkeys: Vec<char>,
}
impl Input {
    fn parse(s: &str) -> Self {
        let mut start = 0;
        let mut end = 0;
        let chars: Vec<_> = s.lines().flat_map(|l| l.chars()).enumerate().map(|(i, c)| {
            match c {
                'S' => {
                    start = i;
                    'a'
                },
                'E' => {
                    end = i;
                    'z'
                },
                x => x
            }
        }).collect();
        Self { monkeys: chars }
    }
}

fn input() -> Input {
    // Array2::from_shape_vec((41, 167), v);
    Input::parse(include_str!("./input.txt"))
}

fn _test() -> Input {
    Input::parse(include_str!("./test.txt"))
}
