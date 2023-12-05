use std::collections::HashSet;

fn main() {
    let input = include_str!("input-1.txt");

    let p1 = part1(input);
    println!("Part 1: {p1:?}");

    let p2 = part2(input);
    println!("Part 2: {p2:?}");
}

fn part2(input: &str) -> Option<usize> {
    let almanac = parse(input)?;
    let seeds: HashSet<usize> = almanac
        .seeds
        .chunks(2)
        .map(|c| (c[0]..=c[0] + c[1]).collect::<HashSet<usize>>())
        .reduce(|acc, s| {
            println!("{s:?}");
            acc.intersection(&s).copied().collect()
        })
        .unwrap();

    let s: Vec<_> = seeds.iter().collect();
    println!("{s:?}");
    Some(9)
    // seeds.iter().map(|s| map_soil_location(*s, &almanac)).min()
}

fn part1(input: &str) -> Option<usize> {
    let almanac = parse(input)?;
    almanac
        .seeds
        .iter()
        .map(|s| map_soil_location(*s, &almanac))
        .min()
}

#[derive(Debug, PartialEq, Eq)]
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

fn map_soil_location(seed: usize, almanac: &Almanac) -> usize {
    let soil = map_value(seed, &almanac.seed_soil_mapping);
    let fertilizer = map_value(soil, &almanac.soil_fertilizer_mapping);
    let water = map_value(fertilizer, &almanac.fertilizer_water_mapping);
    let light = map_value(water, &almanac.water_light_mapping);
    let temperature = map_value(light, &almanac.light_temperature_mapping);
    let humidity = map_value(temperature, &almanac.temperature_humidity_mapping);
    let location = map_value(humidity, &almanac.humidity_location_mapping);
    location
}

fn map_value(value: usize, mapping: &[Mapping]) -> usize {
    println!("{}", mapping.len());
    for m in mapping {
        if value >= m.src_start && value <= m.src_start + m.range_len {
            return m.dest_start + (value - m.src_start);
        }
    }

    value
}

#[derive(Debug, PartialEq, Eq)]
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
    let seed_soil_mapping = gs.next().map(parse_mapping)?;
    let soil_fertilizer_mapping = gs.next().map(parse_mapping)?;
    let fertilizer_water_mapping = gs.next().map(parse_mapping)?;
    let water_light_mapping = gs.next().map(parse_mapping)?;
    let light_temperature_mapping = gs.next().map(parse_mapping)?;
    let temperature_humidity_mapping = gs.next().map(parse_mapping)?;
    let humidity_location_mapping = gs.next().map(parse_mapping)?;

    Some(Almanac {
        seeds,
        seed_soil_mapping,
        soil_fertilizer_mapping,
        fertilizer_water_mapping,
        water_light_mapping,
        light_temperature_mapping,
        temperature_humidity_mapping,
        humidity_location_mapping,
    })
}

fn parse_mapping(g: &str) -> Vec<Mapping> {
    g.lines()
        .skip(1)
        .flat_map(|l| {
            let parts: Vec<_> = l.splitn(3, ' ').collect();
            if parts.len() != 3 {
                return None;
            }

            Some(Mapping {
                dest_start: parts[0].parse().ok()?,
                src_start: parts[1].parse().ok()?,
                range_len: parts[2].parse().ok()?,
            })
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
