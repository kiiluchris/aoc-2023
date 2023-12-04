fn main() {
    let input = include_str!("./input-1.txt");
    let p1 = part1(
        input,
        CubeCount {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    println!("Part 1: {p1}");

    let p2 = part2(input);
    println!("Part 2: {p2}");
}

#[derive(Debug)]
struct CubeCount {
    red: u64,
    blue: u64,
    green: u64,
}

#[derive(Debug)]
struct Game {
    id: u64,
    sets: Vec<CubeCount>,
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .flat_map(|line| {
            let game = parse_game(line)?;
            let (mut red, mut green, mut blue) = (0, 0, 0);

            for set in game.sets {
                blue = std::cmp::max(blue, set.blue);
                green = std::cmp::max(green, set.green);
                red = std::cmp::max(red, set.red);
            }

            Some(red * green * blue)
        })
        .sum()
}

fn part1(input: &str, tgt: CubeCount) -> u64 {
    input
        .lines()
        .flat_map(|line| {
            let game = parse_game(line)?;

            for set in game.sets {
                if set.blue > tgt.blue || set.red > tgt.red || set.green > tgt.green {
                    return None;
                }
            }

            Some(game.id)
        })
        .sum()
}

fn parse_game(line: &str) -> Option<Game> {
    if line.trim() == "" {
        return None;
    }

    let id = line[5..]
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<u64>()
        .ok()?;

    let colon_idx = line.find(':')?;
    let raw_sets = line[colon_idx + 1..].split(';').collect::<Vec<&str>>();
    let mut sets: Vec<_> = Vec::with_capacity(raw_sets.len());

    for s in raw_sets {
        let mut cc = CubeCount {
            red: 0,
            blue: 0,
            green: 0,
        };

        for cube in s.split(',') {
            let cube = cube.trim();
            let (count, color) = cube.split_once(' ')?;
            let count = count.parse::<u64>().ok()?;
            match color {
                "red" => cc.red = count,
                "blue" => cc.blue = count,
                "green" => cc.green = count,

                _ => {}
            }
        }

        sets.push(cc);
    }

    Some(Game { id, sets })
}

#[cfg(test)]
mod tests {
    use crate::{part1, part2, CubeCount};

    #[test]
    fn test_part1() {
        let tgt = CubeCount {
            red: 12,
            green: 13,
            blue: 14,
        };
        assert_eq!(part1(include_str!("test-input-1.txt"), tgt), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("test-input-1.txt")), 2286);
    }
}
