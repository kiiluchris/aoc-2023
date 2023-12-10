fn main() {
    let input = include_str!("./input-1.txt");

    let p1 = part1(input);
    dbg!(p1);

    let p2 = part2(input);
    dbg!(p2);
}

fn part2(input: &str) -> i64 {
    parse(input)
        .into_iter()
        .map(|mut l| {
            l.reverse();
            extrapolate(l)
        })
        .sum()
}

fn part1(input: &str) -> i64 {
    parse(input).into_iter().map(extrapolate).sum()
}

fn extrapolate(line: Vec<i64>) -> i64 {
    let mut diffs = vec![line];

    loop {
        let last = &diffs[diffs.len() - 1];
        if last.iter().all(|c| *c == 0) {
            break;
        }

        let diff = last
            .iter()
            .zip(last.iter().skip(1))
            .map(|(l, r)| *r - *l)
            .collect();
        diffs.push(diff);
    }

    diffs.iter().flat_map(|c| c.last()).sum()
}

fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| l.split_whitespace().flat_map(str::parse::<i64>).collect())
        .collect()
}

#[cfg(test)]
mod day9_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), 2);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), 114);
    }
}
