#![allow(unused)]

use std::collections::HashSet;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let output = input().lines().map(parse_range).filter(|(a, b)| a.full_overlap(&b)).count();
    dbg!(output);
}

fn part_two() {
    let output: usize =
        input().lines().map(parse_range).filter(|(a, b)| a.intersection(&b) > 0).count();
    dbg!(output);
}

struct Range(usize, usize);

impl Range {
    fn full_overlap(&self, other: &Self) -> bool {
        (self.0 <= other.0 && self.1 >= other.1) || (self.0 >= other.0 && self.1 <= other.1)
    }

    fn intersection(&self, other: &Self) -> usize {
        let a: HashSet<_> = (self.0..=self.1).collect();
        let b: HashSet<_> = (other.0..=other.1).collect();
        // let x: Vec<_> = a.intersection(&b).collect();
        // println!("{x:?}");
        a.intersection(&b).count()
    }
}

fn parse_range(s: &str) -> (Range, Range) {
    let (l, r) = s.split_once(',').unwrap();
    let (a, b) = l.split_once('-').unwrap();
    let r1 = Range(a.parse().unwrap(), b.parse().unwrap());
    let (a, b) = r.split_once('-').unwrap();
    let r2 = Range(a.parse().unwrap(), b.parse().unwrap());
    (r1, r2)
}

fn input() -> &'static str {
    include_str!("./input.txt")
}

fn test() -> &'static str {
    include_str!("./test.txt")
}
