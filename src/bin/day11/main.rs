#![allow(unused)]

use std::ops::{Add, Mul};

fn main() {
    part_one();
    // part_two();
}

fn part_one() {
    let mut input = input();
    // for m in &input.monkeys {
    //     println!("{:?}", m);
    // }
    for _ in 0..10_000 {
        input.round();
    }
    for (i, m) in input.monkeys.iter().enumerate() {
        println!("{} : {}", i, m.inspections);
    }
    let mut inspections: Vec<_> = input.monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort_unstable_by(|a, b| b.cmp(a));
    dbg!(inspections[0] * inspections[1]);
}

fn part_two() {}

struct Input {
    monkeys: Vec<Monkey>,
}
impl Input {
    fn parse(s: &str) -> Self {
        let monkeys = s.split("\n\n").map(Monkey::parse).collect();
        Self { monkeys }
    }

    fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let throws = self.monkeys[i].round();
            for (idx, v) in throws {
                self.monkeys[idx].items.push(v)
            }
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u128>,
    operation: Operation,
    test: Test,
    inspections: usize,
}
impl Monkey {
    fn new(items: Vec<u128>, operation: Operation, test: Test) -> Self {
        Self { items, operation, test, inspections: 0 }
    }

    fn parse(s: &str) -> Self {
        let lines: Vec<_> = s.lines().collect();
        assert_eq!(lines.len(), 6);
        let items = String::from_utf8_lossy(&lines[1].as_bytes()[18..])
            .split(", ")
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = Operation::parse(lines[2]);
        let test = Test::parse(lines[3], lines[4], lines[5]);
        Self::new(items, operation, test)
    }

    fn round(&mut self) -> Vec<(usize, u128)> {
        self.inspections += self.items.len();
        let output = self
            .items
            .iter()
            .map(|x| {
                let inspected = self.operation.process(*x);
                // let mid = inspected / 3;
                let mid = inspected % (2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23);
                (self.test.test(mid), mid)
            })
            .collect();
        self.items.clear();
        output
    }
}

#[derive(Debug)]
struct Operation {
    func: fn(u128, u128) -> u128,
    operands: (Operand, Operand),
}
impl Operation {
    fn parse(s: &str) -> Self {
        let mut s = String::from_utf8_lossy(&s.as_bytes()[19..]);
        let mut parts = s.split(' ');
        let a = Operand::parse(parts.next().unwrap());
        let func = match parts.next().unwrap() {
            "+" => Add::add,
            "*" => Mul::mul,
            x => panic!("Invalid str: {x}"),
        };
        let b = Operand::parse(parts.next().unwrap());
        Self { func, operands: (a, b) }
    }

    fn process(&self, x: u128) -> u128 {
        (self.func)(self.operands.0.value(x), self.operands.1.value(x))
    }
}

#[derive(Debug)]
enum Operand {
    Old,
    Val(u128),
}
impl Operand {
    fn parse(s: &str) -> Self {
        match s {
            "old" => Self::Old,
            s => Self::Val(s.parse().unwrap()),
        }
    }

    fn value(&self, x: u128) -> u128 {
        match self {
            Self::Old => x,
            Self::Val(y) => *y,
        }
    }
}

#[derive(Debug)]
struct Test {
    divisible: u128,
    throw: (usize, usize),
}
impl Test {
    fn parse(a: &str, b: &str, c: &str) -> Self {
        // println!("{} : {} : {}", a.len(), b.len(), c.len());
        let divisible = a[21..].parse().unwrap();
        let x = b[29..].parse().unwrap();
        let y = c[30..].parse().unwrap();
        Self { divisible, throw: (x, y) }
    }

    fn test(&self, x: u128) -> usize {
        if x % self.divisible == 0 {
            self.throw.0
        } else {
            self.throw.1
        }
    }
}

fn input() -> Input {
    Input::parse(include_str!("./input.txt"))
}

fn _test() -> Input {
    Input::parse(include_str!("./test.txt"))
}
