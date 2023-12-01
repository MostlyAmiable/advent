use ndarray::Array2;
use regex::Regex;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    input().move9000();
}

fn part_two() {
    input().move9001();
}

struct Input {
    stacks: Vec<Vec<u8>>,
    procedures: Vec<Proc>,
}

impl Input {
    fn move9000(&mut self) {
        for proc in &self.procedures {
            for _ in 0..proc.n {
                let x = self.stacks[proc.from - 1].pop().unwrap();
                self.stacks[proc.to - 1].push(x);
            }
        }
        for s in &mut self.stacks {
            print!("{}", char::from_u32(s.pop().unwrap() as u32).unwrap());
        }
        println!();
    }

    fn move9001(&mut self) {
        for proc in &self.procedures {
            let from = &mut self.stacks[proc.from - 1];
            let idx = from.len() - proc.n;
            let buf: Vec<_> = from.drain(idx..).collect();
            self.stacks[proc.to - 1].extend_from_slice(&buf);
        }
        for s in &mut self.stacks {
            print!("{}", char::from_u32(s.pop().unwrap() as u32).unwrap());
        }
        println!();
    }

    fn parse(s: &str, rows: usize, cols: usize) -> Self {
        let (a, b) = s.split_once("\n\n").unwrap();

        let (a, _) = a.rsplit_once('\n').unwrap();
        let lines = a.lines().flat_map(|s| {
            let b = s.as_bytes();
            (0..cols).map(|i| b[i * 4 + 1]).collect::<Vec<u8>>()
        });
        let arr = Array2::from_shape_vec((rows, cols), lines.collect()).unwrap();
        let stacks = arr
            .columns()
            .into_iter()
            .map(|c| {
                let mut stack: Vec<u8> =
                    c.iter().filter(|&&x| x >= b'A' && x <= b'Z').copied().collect();
                stack.reverse();
                stack
            })
            .collect();

        let pattern = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        let procedures: Vec<_> = b
            .lines()
            .map(|s| {
                let cap = pattern.captures(s).unwrap();
                Proc::new(cap[1].parse().unwrap(), cap[2].parse().unwrap(), cap[3].parse().unwrap())
            })
            .collect();
        Self { stacks, procedures }
    }
}

struct Proc {
    n: usize,
    from: usize,
    to: usize,
}

impl Proc {
    fn new(n: usize, from: usize, to: usize) -> Self {
        Self { n, from, to }
    }
}

fn input() -> Input {
    Input::parse(include_str!("./input.txt"), 8, 9)
}

fn _test() -> Input {
    Input::parse(include_str!("./test.txt"), 3, 3)
}
