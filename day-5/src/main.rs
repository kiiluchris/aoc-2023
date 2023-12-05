fn main() {
    let input = include_str!("input-1.txt");

    let p1 = part1(input);
    println!("Part 1: {p1}");

    let p2 = part2(input);
    println!("Part 2: {p2}");
}

fn part2(input: &str) -> usize {
    0
}

fn part1(input: &str) -> usize {
    0
}

#[cfg(test)]
mod day5_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test-input-1.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test-input-1.txt")), 0);
    }
}
