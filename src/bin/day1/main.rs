#![allow(unused)]

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let calories = input();
    dbg!(calories[calories.len() - 1]);
}

fn part_two() {
    let calories = input();
    dbg!(calories[calories.len() - 3..calories.len()].into_iter().sum::<usize>());
}

fn input() -> Vec<usize> {
    let mut calories: Vec<usize> = include_str!("./input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .collect();
    calories.sort_unstable();
    calories
}
