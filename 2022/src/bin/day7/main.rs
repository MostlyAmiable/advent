#![allow(unused)]
#![feature(slice_group_by)]

use std::collections::BTreeMap;
use std::iter::Peekable;
use std::path::{Path, PathBuf};

const CMD_CD: &str = "$ cd";
const CMD_LS: &str = "$ ls";

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
    let input: Vec<_> = include_str!("input.txt").lines().collect();
    let mut shell = Shell::new();
    let mut cmds = input.group_by(|a, b| !b.starts_with('$')).map(parse_input);
    cmds.next().unwrap();
    cmds.for_each(|cmd| shell.execute(cmd));
}

// fn part_two() {}

struct Shell {
    cwd: PathBuf,
    files: BTreeMap<PathBuf, usize>,
}

impl Shell {
    fn new() -> Self {
        Self {
            cwd: PathBuf::from("/"),
            files: HashMap::new(),
        }
    }

    fn execute(&mut self, cmd: Cmd) {
        match cmd {
            Cmd::Cd(path) => {
                if path == Path::new("..") {
                    assert!(self.cwd.pop());
                } else {
                    self.cwd.push(path);
                }
            }
            Cmd::Ls(entries) => {
                entries.into_iter().for_each(|entry| {
                    if let DirEntry::File(size, path) = entry {
                        self.files.insert(self.cwd.join(path), size);
                    }
                    // match entry {
                    //     DirEntry::Dir(path) => {
                    //         let path = self.cwd.join(path);
                    //         if self.files.contains(&path) {
                    //             println!("{}", path.display());
                    //         }
                    //     }
                    //     DirEntry::File(size, path) => {
                    //         let path = self.cwd.join(path);
                    //         if self.files.contains(&path) {
                    //             println!("{} {}", size, path.display());
                    //         }
                    //     }
                    // }
                });
            }
        }
    }
}

enum Cmd {
    Ls(Vec<DirEntry>),
    Cd(Path),
}

enum DirEntry {
    Dir(Path),
    File(Path, usize),
}

struct Directory {
    path: PathBuf,
    files: Vec<File>,
}

fn parse_input(lines: &[&'static str]) -> Cmd {
    let (cmd, args) = lines[0].split_at(4);
    match cmd {
        CMD_CD => {
            assert_eq!(lines.len(), 1);
            let path = Path::from(args);
            Cmd::Cd(path)
        }
        CMD_LS => {
            assert!(args.is_empty());
            let entries = lines[1..].iter().map(|s| {
                let (size, path) = s.split_once(' ').unwrap();
                if size == "dir" {
                    return DirEntry::Dir(Path::from(path));
                }
                DirEntry::File(size.parse().unwrap(), Path::from(path))
            }).collect();
            Cmd::Ls(entries)
        }
        _ => panic!("Invalid command: {}", lines[0]),
    }
}

fn input() -> Vec<u8> {
    include_str!("./input.txt").as_bytes().to_owned()
}

fn _test() -> Vec<u8> {
    include_str!("./test.txt").as_bytes().to_owned()
}
