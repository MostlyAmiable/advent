#![allow(unused)]

use std::collections::HashSet;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    let x: usize = input().lines().map(parse_compartments).sum();
    dbg!(x);
}

fn part_two() {
    let lines: Vec<_> = input().lines().collect();
    let x: usize = lines.chunks(3).map(parse_group).sum();
    dbg!(x);
}

fn parse_compartments(s: &str) -> usize {
    let b = s.as_bytes();
    let (l, r) = b.split_at(b.len() / 2);
    let l: HashSet<u8> = l.iter().copied().collect();
    let r: HashSet<u8> = r.iter().copied().collect();
    println!("{:?}", l.intersection(&r).collect::<Vec<&u8>>());
    let x = l.intersection(&r);
    x.map(priority).sum()
}

fn parse_group(g: &[&str]) -> usize {
    let mut sets = g.iter().map(|&s| s.bytes().collect::<HashSet<u8>>());
    let first = sets.next().unwrap();
    let diff = sets.fold(first, |a, b| a.intersection(&b).copied().collect());
    assert_eq!(diff.len(), 1);
    priority(&diff.into_iter().next().unwrap())
}

fn priority(c: &u8) -> usize {
    let v = match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("Invalid char: {c}"),
    };
    v as usize
}

fn input() -> &'static str {
    include_str!("./input.txt")
}

fn test() -> &'static str {
    include_str!("./test.txt")
}
