fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let calories = input();
    dbg!(calories[0]);
}

fn part_two() {
    let calories = input();
    dbg!(calories[..3].into_iter().sum::<usize>());
}

fn input() -> Vec<usize> {
    let mut calories: Vec<usize> = include_str!("./input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|s| s.parse::<usize>().unwrap()).sum())
        .collect();
    calories.sort_unstable_by(|a, b| b.cmp(a)); // Sort descending
    calories
}
