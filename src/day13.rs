use atoi_simd;
use memchr::memchr;

type Input<'a> = &'a [u8];
type Output = usize;

struct Game {
    button_a: (isize, isize),
    button_b: (isize, isize),
    prize: (isize, isize),
    offset: usize,
}

impl Game {
    fn solve_equation(&self, offset: isize) -> Option<(usize, usize)> {
        let prize = (self.prize.0 + offset, self.prize.1 + offset);
        let det = self.button_a.0 * self.button_b.1 - self.button_a.1 * self.button_b.0;
        let a = (prize.0 * self.button_b.1 - prize.1 * self.button_b.0) / det;
        let b = (self.button_a.0 * prize.1 - self.button_a.1 * prize.0) / det;
        if (
            self.button_a.0 * a + self.button_b.0 * b,
            self.button_a.1 * a + self.button_b.1 * b,
        ) == (prize.0, prize.1)
        {
            Some((a as usize, b as usize))
        } else {
            None
        }
    }

    unsafe fn parse_input(input: Input, offset: usize) -> Self {
        /*
            Button A: X+94, Y+34
            Button B: X+22, Y+67
            Prize: X=8400, Y=5400
        */
        let mut from = 12 + offset;
        let mut to = memchr(b',', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let button_ax: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        from = to + 4;
        to = memchr(b'\n', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let button_ay: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        from = to + 13;
        to = memchr(b',', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let button_bx: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        from = to + 4;
        to = memchr(b'\n', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let button_by: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        from = to + 10;
        to = memchr(b',', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let prizex: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        from = to + 4;
        to = memchr(b'\n', input.get_unchecked(from..)).unwrap_unchecked() + from;
        let prizey: usize = atoi_simd::parse(input.get_unchecked(from..to)).unwrap_unchecked();
        Self {
            button_a: (button_ax as isize, button_ay as isize),
            button_b: (button_bx as isize, button_by as isize),
            prize: (prizex as isize, prizey as isize),
            offset: to + 1,
        }
    }
}

fn solve(input: &str, limit: Option<usize>, offset: isize) -> Output {
    let input = input.as_bytes();
    let mut sum = 0;
    let mut cursor = 0;
    while cursor < input.len() {
        let game = unsafe { Game::parse_input(input, cursor) };
        cursor = game.offset + 1;

        if let Some((a, b)) = game.solve_equation(offset) {
            if let Some(limit) = limit {
                if a > limit || b > limit {
                    continue;
                }
            }
            let tokens = 3 * a + b;
            sum += tokens;
        }
    }
    sum
}

#[aoc(day13, part1)]
pub fn part1(input: &str) -> Output {
    solve(input, Some(100), 0)
}

#[aoc(day13, part2)]
pub fn part2(input: &str) -> Output {
    solve(input, None, 10_000_000_000_000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279\n";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output) = (480, 875_318_608_908);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day13.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (29023, 96_787_395_375_634);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
