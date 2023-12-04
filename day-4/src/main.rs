use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("./input-1.txt");

    let p1 = part1(input);
    println!("Part 1: {p1}");

    let p2 = part2(input);
    println!("Part 2: {p2}");
}

struct Card {
    winning: HashSet<u64>,
    owned: Vec<u64>,
}

impl Card {
    fn winning_number_count(&self) -> usize {
        self.owned
            .iter()
            .filter(|o| self.winning.contains(o))
            .count()
    }

    fn points(&self) -> usize {
        let owned_winning_numbers = self.winning_number_count();

        if owned_winning_numbers == 0 {
            return owned_winning_numbers;
        }

        usize::pow(2, (owned_winning_numbers - 1).try_into().unwrap())
    }
}

fn part2(input: &str) -> usize {
    let cards: Vec<Card> = input.lines().flat_map(parse_card).collect();
    let last_card = cards.len();
    let card_counts: HashMap<usize, usize> = (1..=last_card).map(|i| (i, 0)).collect();
    let card_counts = cards.iter().zip(1..).fold(card_counts, |mut acc, (c, id)| {
        let count = acc.entry(id).or_insert(0);
        *count += 1;

        let count: usize = *count;
        for i in id + 1..=(c.winning_number_count() + id).min(last_card) {
            *acc.entry(i).or_insert(0) += count;
        }

        acc
    });

    card_counts.into_values().sum()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .flat_map(|l| {
            let card = parse_card(l)?;
            Some(card.points())
        })
        .sum()
}

fn parse_card(input: &str) -> Option<Card> {
    let (_, input) = input.split_once(':')?;
    let (winning, owned) = input.split_once('|')?;
    let winning: HashSet<u64> = winning.split(' ').flat_map(str::parse::<u64>).collect();
    let owned: Vec<u64> = owned.split(' ').flat_map(str::parse::<u64>).collect();
    Some(Card { winning, owned })
}

#[cfg(test)]
mod day4_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), 30);
    }
}
