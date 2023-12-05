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
    parse(input).unwrap();
    0
}

struct Almanac {
    seeds: Vec<usize>,
    seed_soil_mapping: Vec<Mapping>,
    soil_fertilizer_mapping: Vec<Mapping>,
    fertilizer_water_mapping: Vec<Mapping>,
    water_light_mapping: Vec<Mapping>,
    light_temperature_mapping: Vec<Mapping>,
    temperature_humidity_mapping: Vec<Mapping>,
    humidity_location_mapping: Vec<Mapping>,
}

struct Mapping {
    dest_start: usize,
    src_start: usize,
    range_len: usize,
}

fn parse(input: &str) -> Option<Almanac> {
    let mut gs = input.splitn(8, "\n\n");
    let first = gs.next()?;
    let (_, first) = first.split_once(": ")?;
    let seeds: Vec<_> = first.split(' ').flat_map(str::parse::<usize>).collect();

    None
}

#[cfg(test)]
mod day5_tests {
    use crate::{part1, part2};

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./test-input-1.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./test-input-1.txt")), 0);
    }
}
