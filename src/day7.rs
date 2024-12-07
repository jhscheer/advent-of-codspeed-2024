type Input<'a> = &'a str;
type Output = usize;

fn parse_input(s: &str) -> impl Iterator<Item = Vec<usize>> + '_ {
    s.lines().map(|line| {
        line.split(|c: char| !c.is_ascii_alphanumeric())
            .filter_map(|v| v.parse().ok())
            .collect()
    })
}

fn is_valid1(target: usize, nums: &[usize]) -> bool {
    if nums.is_empty() {
        return target == 0;
    }
    if target == 0 {
        return false;
    }
    is_valid1(
        target.wrapping_sub(unsafe { *nums.last().unwrap_unchecked() }),
        unsafe { nums.get_unchecked(..(nums.len() - 1)) },
    ) || (target % unsafe { nums.last().unwrap_unchecked() } == 0
        && is_valid1(target / unsafe { nums.last().unwrap_unchecked() }, unsafe {
            nums.get_unchecked(..(nums.len() - 1))
        }))
}

fn is_valid2(target: usize, current: usize, nums: &[usize]) -> Option<usize> {
    match nums {
        [] if current == target => Some(target),
        [head, tail @ ..] if current <= target => {
            let p = 1 + head.ilog10();
            let num = current * 10usize.pow(p) + head;
            is_valid2(target, num, tail)
                .or_else(|| is_valid2(target, current + head, tail))
                .or_else(|| is_valid2(target, current * head, tail))
        }
        _ => None,
    }
}

#[aoc(day7, part1)]
pub fn part1(input: Input) -> Output {
    parse_input(input)
        .filter(|ns| is_valid1(ns[0], &ns[1..]))
        .fold(0, |acc, ns| acc + ns[0])
}

#[aoc(day7, part2)]
pub fn part2(input: Input) -> Output {
    parse_input(input)
        .filter(|ns| ns.len() > 1 && is_valid2(ns[0], ns[1], &ns[2..]).is_some())
        .fold(0, |acc, ns| acc + ns[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output) = (3749, 11387);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day7.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (4998764814652, 37598910447546);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
