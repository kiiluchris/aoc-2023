use std::collections::HashMap;

fn main() {
    let input = include_str!("./test-input-1.txt");
    let p1 = part1(input);
    dbg!(p1);
}

fn part1(input: &str) -> u64 {
    let plays = parse(input);
    for p in plays {
        println!("{p:?} {s}", s = p.strength());
    }
    0
}

#[derive(Debug, PartialEq)]
struct Play<'a> {
    hand: &'a str,
    bid: u64,
}

impl<'a> Ord for Play<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.strength().cmp(&other.strength()) {
            std::cmp::Ordering::Equal => self.hand.ch
            o => o,
        }
    }
}

const CARD_STRENGTH: [(char, i32); 13] = [
    ('A', 13),
    ('K', 12),
    ('Q', 11),
    ('J', 10),
    ('T', 9),
    ('9', 8),
    ('8', 7),
    ('7', 6),
    ('6', 5),
    ('5', 4),
    ('4', 3),
    ('3', 2),
    ('2', 1),
];

impl<'a> Play<'a> {
    pub fn new(hand: &'a str, bid: u64) -> Self {
        Self { hand, bid }
    }

    pub fn strength(&self) -> u64 {
        let m = self.hand.chars().fold(HashMap::new(), |mut acc, ch| {
            *acc.entry(ch).or_insert(0) += 1u64;
            acc
        });

        match m.len() {
            5 => return 5,
            1 => return 50,
            2 => match m.values().next().unwrap() {
                1 | 4 => return 40,
                2 | 3 => return 35,
                _ => unreachable!(),
            },
            3 => return if m.values().any(|c| *c == 3) { 30 } else { 20 },
            4 => {
                if m.values().any(|c| *c == 2) {
                    return 10;
                }
            }
            _ => {}
        }

        return 0u64;
    }
}

fn parse(input: &str) -> Vec<Play> {
    input
        .lines()
        .flat_map(|l| {
            let (hand, bid) = l.split_once(' ')?;
            let bid = bid.parse().ok()?;
            Some(Play { hand, bid })
        })
        .collect()
}

#[cfg(test)]
mod day7_tests {
    use crate::{parse, Play};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse(include_str!("./test-input-1.txt")),
            vec![
                Play::new("32T3K", 765),
                Play::new("T55J5", 684),
                Play::new("KK677", 28),
                Play::new("KTJJT", 220),
                Play::new("QQQJA", 483),
            ]
        );
    }
}
