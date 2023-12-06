use std::ops::Range;

fn main() {
    let input = include_str!("input-1.txt");

    let p1 = part1(input);
    println!("Part 1: {p1:?}");

    let p2 = part2(input);
    println!("Part 2: {p2:?}");
}

fn part2(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse(input)?;
    let seed_ranges: Vec<_> = seeds.chunks(2).map(|c| (c[0]..c[0] + c[1])).collect();
    merge_ranges(seed_ranges)
        .into_iter()
        .flatten()
        .map(|seed| mappings.iter().fold(seed, |value, m| translate(value, &m)))
        .min()
}

fn merge_ranges(mut ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    ranges.sort_unstable_by_key(|r| r.start);

    let mut idx = 0;
    while idx < ranges.len() - 1 {
        // no overlap
        if ranges[idx].end < ranges[idx + 1].start {
            idx += 1;
            continue;
        }

        ranges[idx].end = ranges[idx + 2].end;
        ranges.remove(idx + 1);
    }

    ranges
}

fn part1(input: &str) -> Option<u64> {
    let (seeds, mappings) = parse(input)?;
    seeds
        .iter()
        .map(|seed| (mappings.iter().fold(*seed, |value, m| translate(value, &m))))
        .min()
}

fn translate(seed: u64, mapping: &[(Range<u64>, Range<u64>)]) -> u64 {
    let found_range = mapping.iter().find(|(src, _)| src.contains(&seed));
    if let Some((src, dest)) = found_range {
        return dest.start + (seed - src.start);
    }

    return seed;
}

fn parse(input: &str) -> Option<(Vec<u64>, Vec<Vec<(Range<u64>, Range<u64>)>>)> {
    let mut gs = input.splitn(8, "\n\n");
    let first = gs.next()?;
    let (_, first) = first.split_once(": ")?;
    let seeds: Vec<_> = first.split(' ').flat_map(str::parse::<u64>).collect();
    let mappings: Vec<_> = gs.map(parse_mapping).collect();

    Some((seeds, mappings))
}

fn parse_mapping(g: &str) -> Vec<(Range<u64>, Range<u64>)> {
    g.lines()
        .skip(1)
        .flat_map(|l| {
            let parts: Vec<_> = l.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return None;
            }

            let dest_start: u64 = parts[0].parse().ok()?;
            let src_start: u64 = parts[1].parse().ok()?;
            let range_len: u64 = parts[2].parse().ok()?;

            Some((
                src_start..src_start + range_len,
                dest_start..dest_start + range_len,
            ))
        })
        .collect()
}

#[cfg(test)]
mod day5_tests {
    use crate::{parse, part1, part2};

    #[test]
    fn test_parse() {
        assert_ne!(parse(include_str!("./test-input-1.txt")), None);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), Some(35));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), Some(46));
    }
}
