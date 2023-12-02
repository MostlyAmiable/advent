fn main() {
    let p1 = part_one(input()); // 2377
    println!("Part one: {}", p1);

    let p2 = part_two(input()); // 71220
    println!("Part two: {}", p2);
}

fn part_one(lines: impl Iterator<Item = &'static str>) -> usize {
    let limit = Bag { red: 12, green: 13, blue: 14 };
    lines
        .map(parse_game)
        .enumerate()
        .filter_map(|(i, bag)| if limit.contains(&bag) { Some(i + 1) } else { None })
        .sum()
}

fn part_two(lines: impl Iterator<Item = &'static str>) -> usize {
    lines.map(parse_game).map(Bag::power).sum()
}

#[derive(Default, Debug)]
struct Bag {
    red: usize,
    green: usize,
    blue: usize,
}
impl Bag {
    fn contains(&self, other: &Self) -> bool {
        self.red >= other.red && self.green >= other.green && self.blue >= other.blue
    }

    fn power(self) -> usize {
        self.red * self.green * self.blue
    }
}

fn parse_game(line: &'static str) -> Bag {
    let mut bag = Bag::default();
    let (_, rounds) = line.split_once(": ").unwrap();
    rounds.split("; ").for_each(|round| {
        round.split(", ").for_each(|group| {
            let (count, color) = group.split_once(" ").unwrap();
            let bag_color = match color {
                "red" => &mut bag.red,
                "green" => &mut bag.green,
                "blue" => &mut bag.blue,
                _ => unreachable!(),
            };
            *bag_color = count.parse::<usize>().unwrap().max(*bag_color);
        });
    });
    bag
}

fn input() -> impl Iterator<Item = &'static str> {
    include_str!("input.txt").lines()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        let input = include_str!("test1.txt").lines();
        let actual = super::part_one(input);
        assert_eq!(8, actual);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test1.txt").lines();
        let actual = super::part_two(input);
        assert_eq!(2286, actual);
    }
}
