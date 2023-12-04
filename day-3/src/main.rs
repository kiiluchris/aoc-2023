fn main() {
    let input = include_str!("./input-1.txt");

    let p1 = part1(input);
    println!("Part 1: {p1:?}");

    let p2 = part2(input);
    println!("Part 2: {p2:?}");
}

fn part2(input: &str) -> Option<usize> {
    let (numbers, symbols) = parse_schematic(input)?;

    let mut sum = 0;

    for g in symbols.iter().filter(|sym| sym.value == "*") {
        let numbers = numbers
            .iter()
            .filter(|n| g.is_adjacent(n))
            .collect::<Vec<_>>();

        if numbers.len() != 2 {
            continue;
        }

        sum += numbers[0].value * numbers[1].value;
    }

    Some(sum)
}

fn part1(input: &str) -> Option<usize> {
    let (numbers, symbols) = parse_schematic(input)?;

    let mut sum = 0;
    for n in numbers {
        if symbols.iter().any(|sym| sym.is_adjacent(&n)) {
            sum += n.value;
        }
    }

    Some(sum)
}

#[derive(Debug)]
struct NumberToken {
    row: usize,
    col_start: usize,
    col_end: usize,
    value: usize,
}

#[derive(Debug, Clone)]
struct SymbolToken {
    row: usize,
    col: usize,
    value: String,
}

impl SymbolToken {
    fn is_adjacent(&self, n: &NumberToken) -> bool {
        self.is_adjacent_row(n.row) && self.is_adjacent_col(n.col_start, n.col_end)
    }

    fn is_adjacent_row(&self, row: usize) -> bool {
        let start = row.saturating_sub(1);
        let end = row.saturating_add(1);
        self.row >= start && self.row <= end
    }

    fn is_adjacent_col(&self, start: usize, end: usize) -> bool {
        let start = start.saturating_sub(1);
        let end = end.saturating_add(1);
        self.col >= start && self.col <= end
    }
}

fn parse_schematic(input: &str) -> Option<(Vec<NumberToken>, Vec<SymbolToken>)> {
    let mut numbers = Vec::new();
    let mut symbols: Vec<SymbolToken> = Vec::new();

    for (row, l) in input.lines().enumerate() {
        let l = l.trim();
        let chars: Vec<_> = l.chars().collect();

        let mut i: usize = 0;
        while i < l.len() {
            let ch = chars[i];
            if ch.is_ascii_digit() {
                let number = chars[i..]
                    .iter()
                    .take_while(|c| c.is_ascii_digit())
                    .collect::<String>();
                numbers.push(NumberToken {
                    col_start: i,
                    col_end: i + number.len() - 1,
                    row,
                    value: number.parse::<usize>().ok()?,
                });
                i += number.len();
                continue;
            } else if ch == '.' {
            } else {
                symbols.push(SymbolToken {
                    row,
                    col: i,
                    value: chars[i].into(),
                });
            }

            i += 1;
        }
    }

    Some((numbers, symbols))
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("test-input-1.txt")), Some(4361));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test-input-1.txt")), Some(467835));
    }
}
