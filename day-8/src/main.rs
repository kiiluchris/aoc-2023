use std::collections::HashMap;

fn main() {
    let input = include_str!("./input-1.txt");

    let p1 = part1(input);
    dbg!(p1);

    let p2 = part2(input);
    dbg!(p2);
}

fn part2(input: &str) -> u64 {
    let net = parse(input);

    net.nodes
        .keys()
        .flat_map(|k| {
            if !k.ends_with('A') {
                return None;
            }

            Some(find_end(&net, k, |n| n.ends_with('Z')))
        })
        .fold(1, num::integer::lcm)
}

fn part1(input: &str) -> u64 {
    let net = parse(input);
    find_end(&net, "AAA", |n| n == "ZZZ")
}

fn find_end<F>(net: &Network, start_node: &str, end_node: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let mut steps = 0;
    let mut current_node = start_node;
    for i in net.instructions.iter().cycle() {
        if end_node(current_node) {
            break;
        }

        let node = net.nodes.get(current_node).unwrap();
        current_node = if *i == Instruction::Left {
            node.left
        } else {
            node.right
        };
        steps += 1;
    }

    steps
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Network<'a> {
    instructions: Vec<Instruction>,
    nodes: HashMap<&'a str, Node<'a>>,
}

#[derive(Debug, Clone, Copy)]
struct Node<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse(input: &str) -> Network {
    let mut lines = input.lines();
    let instructions = lines
        .next()
        .expect("failed to get instructions line")
        .chars()
        .map(|c| {
            if c == 'L' {
                Instruction::Left
            } else {
                Instruction::Right
            }
        })
        .collect();

    let nodes = lines
        .flat_map(|l| {
            let (name, out) = l.split_once(" = (")?;
            let (left, right) = out.split_once(", ")?;
            let right = right.trim_end_matches(")");

            Some((name, Node { left, right }))
        })
        .collect();

    Network {
        instructions,
        nodes,
    }
}

#[cfg(test)]
mod day8_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-3.txt")), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), 2);
        assert_eq!(part1(include_str!("./test-input-2.txt")), 6);
    }
}
