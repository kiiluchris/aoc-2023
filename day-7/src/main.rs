use std::{cmp::Ordering, collections::HashMap, ops::Deref};

use itertools::Itertools;

fn main() {
    let input = include_str!("./input-1.txt");

    let p1 = part1(input);
    dbg!(p1);

    let p2 = part2(input);
    dbg!(p2);
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let bid = bid.parse().ok()?;
            Some((hand, bid, hand_type_p2(hand), hand_scores_p2(hand)))
        })
        .sorted_by(hand_sort)
        .zip(1..)
        .map(|(p, r)| r * p.1)
        .sum()
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let bid = bid.parse().ok()?;
            Some((hand, bid, hand_type_p1(hand), hand_scores_p1(hand)))
        })
        .sorted_by(hand_sort)
        .zip(1..)
        .map(|(p, r)| r * p.1)
        .sum()
}

fn hand_sort(l: &(&str, u64, HandType, Vec<u64>), r: &(&str, u64, HandType, Vec<u64>)) -> Ordering {
    match l.2.cmp(&r.2) {
        Ordering::Equal => {
            for (l, r) in l.3.iter().zip(&r.3) {
                match l.cmp(&r) {
                    Ordering::Equal => continue,
                    o => return o,
                }
            }
            return Ordering::Equal;
        }
        o => o,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Play<'a> {
    hand: &'a str,
    bid: u64,
    score: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn hand_type_p2(hand: &str) -> HandType {
    use HandType::*;

    let mut m = hand.chars().counts();
    let j = m.remove(&'J');

    let mut values: Vec<_> = m.into_values().collect();
    values.sort();
    let len = values.len();

    if let Some(j) = j {
        if j == 5 {
            values.push(j);
        } else {
            values[len - 1] = values[len - 1] + j;
        }
    }

    let values = values.iter().map(|c| c.to_string()).join("");

    match values.deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        t => panic!("unknown hand type {t}"),
    }
}

fn hand_type_p1(hand: &str) -> HandType {
    use HandType::*;

    let m = hand.chars().fold(HashMap::new(), |mut acc, ch| {
        *acc.entry(ch).or_insert(0) += 1u64;
        acc
    });
    let mut values: Vec<_> = m.values().map(u64::to_string).collect();
    values.sort();

    match values.join("").deref() {
        "5" => FiveOfAKind,
        "14" => FourOfAKind,
        "23" => FullHouse,
        "113" => ThreeOfAKind,
        "122" => TwoPair,
        "1112" => OnePair,
        "11111" => HighCard,
        _ => unreachable!(),
    }
}

fn hand_scores_p2(hand: &str) -> Vec<u64> {
    hand.chars()
        .map(|c| match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            'J' => 0,
            _ => unreachable!(),
        })
        .collect()
}

fn hand_scores_p1(hand: &str) -> Vec<u64> {
    hand.chars()
        .map(|c| match c {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => unreachable!(),
        })
        .collect()
}

#[cfg(test)]
mod day7_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), 5905,);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), 6440,);
    }
}
