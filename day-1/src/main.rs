fn main() {
    let input = include_str!("input-1.txt");

    let p1 = part1(input);
    println!("Part 1 Result {p1}");

    let p2 = part2(input);
    println!("Part 2 Result {p2}");
}

const LOWEST_DIGIT: u64 = '0' as u64;
const DIGITS: [(&'static str, u64); 18] = [
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn part2(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|l| {
            let first = find_first(l)?;
            let last = find_last(l)?;
            Some(first * 10 + last)
        })
        .sum()
}

fn find_first(l: &str) -> Option<u64> {
    for i in 0..l.len() {
        let remainder = &l[i..];

        for (digit, v) in DIGITS {
            if remainder.len() < digit.len() {
                continue;
            }

            if &remainder[..digit.len()] == digit {
                return Some(v);
            }
        }
    }

    None
}

fn find_last(l: &str) -> Option<u64> {
    for i in (0..l.len()).rev() {
        let remainder = &l[i..];

        for (digit, v) in DIGITS {
            if remainder.len() < digit.len() {
                continue;
            }

            if &remainder[..digit.len()] == digit {
                return Some(v);
            }
        }
    }

    None
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|l| {
            let first = l.chars().find(char::is_ascii_digit).map(char_to_digit)?;
            let last = l.chars().rfind(char::is_ascii_digit).map(char_to_digit)?;
            Some(first * 10 + last)
        })
        .sum()
}

fn char_to_digit(c: char) -> u64 {
    c as u64 - LOWEST_DIGIT
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test-input-1.txt")), 142);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test-input-2.txt")), 281);
    }
}
