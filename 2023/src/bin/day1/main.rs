fn main() {
    let p1 = part_one(input()); // 55002
    println!("Part one: {}", p1);

    let p2 = part_two(input()); // 55093
    println!("Part two: {}", p2);
}

fn part_one(lines: impl Iterator<Item = &'static str>) -> u32 {
    lines
        .map(|line| {
            let mut chars = line.chars().filter_map(|c| c.to_digit(10));
            let first = chars.next().unwrap();
            let last = chars.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn part_two(lines: impl Iterator<Item = &'static str>) -> usize {
    lines
        .map(str::as_bytes)
        .map(|line| {
            let mut digits = (0..line.len()).filter_map(|start| parse_digit(&line[start..]));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            first * 10 + last
        })
        .sum()
}

fn parse_digit(s: &[u8]) -> Option<usize> {
    const DIGITS: [&str; 10] =
        ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    if s[0].is_ascii_digit() {
        Some((s[0] - b'0') as usize)
    } else {
        for (i, digit) in DIGITS.iter().enumerate() {
            if s.starts_with(digit.as_bytes()) {
                return Some(i);
            }
        }
        None
    }
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
        assert_eq!(142, actual);
    }

    #[test]
    fn test_part_two() {
        let input = include_str!("test2.txt").lines();
        let actual = super::part_two(input);
        assert_eq!(281, actual);
    }

    // fn input() -> impl Iterator<Item = &'static str> {
    //     include_str!("./test.txt").lines()
    // }
}
