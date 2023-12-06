fn main() {
    let input = include_str!("./input-1.txt");

    let part1 = part1(input);
    dbg!(part1);

    let part2 = part2(input);
    dbg!(part2);
}

fn part2(input: &str) -> Option<u64> {
    let (time, distance) = parse2(input)?;

    Some((1..=time).filter(|t| (time - t) * t > distance).count() as u64)
}

fn parse2(input: &str) -> Option<(u64, u64)> {
    let mut lines = input.lines();
    let time = parse_line2(lines.next()?)?;
    let distance = parse_line2(lines.next()?)?;

    Some((time, distance))
}

fn parse_line2(line: &str) -> Option<u64> {
    let (_, numbers) = line.split_once(':')?;
    let numbers = numbers
        .split_whitespace()
        .collect::<String>()
        .parse()
        .ok()?;
    Some(numbers)
}

fn part1(input: &str) -> Option<u64> {
    let races = parse(input)?;
    let result = races
        .into_iter()
        .map(|(time, distance)| (1..=time).filter(|t| (time - t) * t > distance).count() as u64)
        .fold(1, |acc, c| acc * dbg!(c));

    Some(result)
}

fn parse(input: &str) -> Option<Vec<(u64, u64)>> {
    let mut lines = input.lines();
    let times = parse_line(lines.next()?)?;
    let distances = parse_line(lines.next()?)?;
    Some(times.into_iter().zip(distances).collect())
}

fn parse_line(line: &str) -> Option<Vec<u64>> {
    let (_, numbers) = line.split_once(':')?;
    let numbers: Vec<u64> = numbers
        .split_whitespace()
        .flat_map(str::parse::<u64>)
        .collect();
    Some(numbers)
}

#[cfg(test)]
mod day6_tests {
    use crate::{parse, parse2, part1, part2};

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), Some(71503),);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), Some(288),);
    }

    #[test]
    fn test_parse2() {
        assert_eq!(
            parse2(include_str!("./test-input-1.txt")),
            Some((71530, 940200)),
        );
    }

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(include_str!("./test-input-1.txt")),
            Some(vec![(7, 9), (15, 40), (30, 200)]),
        );
    }
}
