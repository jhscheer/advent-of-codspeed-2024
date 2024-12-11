use std::collections::HashMap;

type Input<'a> = &'a str;
type Output = usize;

fn parse_input(input: Input) -> Vec<usize> {
    input[..input.len() - 1]
        .split(" ")
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn count((stone, blinks): (usize, usize), cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(&v) = cache.get(&(stone, blinks)) {
        return v;
    }

    let result = if stone == 0 {
        count((1, blinks - 1), cache)
    } else {
        let s = stone.to_string();
        if s.len() % 2 == 0 {
            let h = s.len() / 2;
            count((s[..h].parse().unwrap(), blinks - 1), cache)
                + count((s[h..].parse().unwrap(), blinks - 1), cache)
        } else {
            count((stone * 2024, blinks - 1), cache)
        }
    };
    let _ = cache.insert((stone, blinks), result);
    result
}

fn solve(input: &str, blinks: usize) -> Output {
    let stones = parse_input(input);
    let mut cache = HashMap::with_capacity(4096);
    stones
        .iter()
        .fold(0, |acc, &stone| acc + count((stone, blinks), &mut cache))
}

#[aoc(day11, part1)]
pub fn part1(input: Input) -> Output {
    solve(input, 25)
}

#[aoc(day11, part2)]
pub fn part2(input: Input) -> Output {
    solve(input, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmpls() {
        const EXAMPLE: &str = "125 17\n";
        const EXPECTED: Output = 55312;
        assert_eq!(part1(EXAMPLE), EXPECTED);
    }

    const INPUT: &str = include_str!("../input/2024/day11.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (189167, 225253278506288);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
