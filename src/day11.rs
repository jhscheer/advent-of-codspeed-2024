use arrayvec::ArrayVec;
use atoi_simd;
use fxhash::FxHashMap;
use itoa;

const MAX_LEN: usize = 32;
const DEFAULT_CAPACITY: usize = 4096;

unsafe fn parse_input(input: &str) -> ArrayVec<usize, MAX_LEN> {
    input[..input.len() - 1]
        .split(" ")
        .map(|s| atoi_simd::parse(s.as_bytes()).unwrap_unchecked())
        .collect()
}

unsafe fn count(
    (stone, blinks): (usize, usize),
    cache: &mut FxHashMap<(usize, usize), usize>,
) -> usize {
    if blinks == 0 {
        return 1;
    }
    if let Some(&v) = cache.get(&(stone, blinks)) {
        return v;
    }

    let result = if stone == 0 {
        count((1, blinks - 1), cache)
    } else {
        let mut buffer = itoa::Buffer::new();
        let s = buffer.format(stone);

        if s.len() % 2 == 0 {
            let h = s.len() / 2;
            count(
                (
                    atoi_simd::parse(s[..h].as_bytes()).unwrap_unchecked(),
                    blinks - 1,
                ),
                cache,
            ) + count(
                (
                    atoi_simd::parse(s[h..].as_bytes()).unwrap_unchecked(),
                    blinks - 1,
                ),
                cache,
            )
        } else {
            count((stone * 2024, blinks - 1), cache)
        }
    };
    let _ = cache.insert((stone, blinks), result);
    result
}

unsafe fn solve(input: &str, blinks: usize) -> usize {
    let stones = parse_input(input);
    let mut cache = FxHashMap::with_capacity_and_hasher(DEFAULT_CAPACITY, Default::default());
    stones
        .iter()
        .fold(0, |acc, &stone| acc + count((stone, blinks), &mut cache))
}

#[aoc(day11, part1)]
pub fn part1(input: &str) -> usize {
    unsafe { solve(input, 25) }
}

#[aoc(day11, part2)]
pub fn part2(input: &str) -> usize {
    unsafe { solve(input, 75) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xmpls() {
        const EXAMPLE: &str = "125 17\n";
        const EXPECTED: usize = 55312;
        assert_eq!(part1(EXAMPLE), EXPECTED);
    }

    const INPUT: &str = include_str!("../input/2024/day11.txt");

    #[test]
    fn input() {
        const EXPECTED: (usize, usize) = (189167, 225253278506288);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
