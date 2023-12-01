#![allow(unused)]

use std::collections::HashSet;
use std::iter::Peekable;
use std::path::PathBuf;

fn main() {
    // part_one();
    part_two();
}

fn part_one() {
    // dbg!(Input::new().run(input(), 4));
    // include_str!("./test.txt").lines().for_each(|s| {
    //     dbg!(Input::new().run(s.as_bytes().to_owned()));
    // });
}

fn part_two() {
    // dbg!(Input::new().run(input(), 14));
}

struct Input {
    buffer: Vec<u8>,
}

impl Input {}

enum Cmd {
    Ls(Vec<DirEntry>),
    Cd(PathBuf),
}

enum DirEntry {
    Dir(PathBuf),
    File(usize, PathBuf),
}

fn parse_input<'a>(mut iter: Peekable<impl Iterator<Item = &'a str>>) -> Option<Cmd> {
    let s = iter.next()?;
    // if s[..4] == "$ cd" {
    if s == "$ ls" {
        // while
        todo!()
    } else {
        todo!()
    }
}

fn input() -> Vec<u8> {
    include_str!("./input.txt").as_bytes().to_owned()
}

fn _test() -> Vec<u8> {
    include_str!("./test.txt").as_bytes().to_owned()
}
