use arrayvec::ArrayVec;
use atoi_simd;
use rustc_hash::FxHashSet;

const MAX_LEN: usize = 1 << 9;
type Point = (isize, isize);
type Input<'a> = &'a str;
type Parsed = ArrayVec<(Point, Point), MAX_LEN>;
type Output = usize;

unsafe fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        // p=0,4 v=3,-3
        .map(|l| l.split_once(" ").unwrap_unchecked())
        .map(|(a, b)| {
            (
                a.split_once("=").unwrap_unchecked(),
                b.split_once("=").unwrap_unchecked(),
            )
        })
        .map(|((_, a), (_, b))| {
            (
                a.split_once(",").unwrap_unchecked(),
                b.split_once(",").unwrap_unchecked(),
            )
        })
        .map(|((px, py), (vx, vy))| {
            (
                (
                    atoi_simd::parse(px.as_bytes()).unwrap_unchecked(),
                    atoi_simd::parse(py.as_bytes()).unwrap_unchecked(),
                ),
                (
                    atoi_simd::parse(vx.as_bytes()).unwrap_unchecked(),
                    atoi_simd::parse(vy.as_bytes()).unwrap_unchecked(),
                ),
            )
        })
        .collect()
}

#[aoc(day14, part1)]
pub fn part1(input: Input) -> Output {
    let input = unsafe { parse_input(input) };
    let mut quadrant = [0, 0, 0, 0];

    for &((px, py), (vx, vy)) in &input {
        let nx = (px + 100 * vx).rem_euclid(101);
        let ny = (py + 100 * vy).rem_euclid(103);

        if nx < 50 && ny < 51 {
            quadrant[0] += 1;
        } else if nx > 50 && ny < 51 {
            quadrant[1] += 1;
        } else if nx < 50 && ny > 51 {
            quadrant[2] += 1;
        } else if nx > 50 && ny > 51 {
            quadrant[3] += 1;
        }
    }
    quadrant.iter().product()
}

const STEPS_MAX: isize = 1 << 14;

#[aoc(day14, part2)]
pub fn part2(input: Input) -> Output {
    let input = unsafe { parse_input(input) };
    let total = input.len();

    let mut seen = FxHashSet::with_capacity_and_hasher(total, Default::default());
    for steps in 0..STEPS_MAX {
        seen.clear();

        for &((px, py), (vx, vy)) in &input {
            let nx = (px + steps * vx).rem_euclid(101);
            let ny = (py + steps * vy).rem_euclid(103);

            let n = (nx, ny);
            let _ = seen.insert(n);
        }
        if seen.len() == total {
            return steps as usize;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3\n";

    #[test]
    fn xmpls() {
        const EXPECTED: Output = 21;
        assert_eq!(part1(EXAMPLE), EXPECTED);
    }

    const INPUT: &str = include_str!("../input/2024/day14.txt");

    #[test]
    fn input() {
        const SOLUTION: (Output, Output) = (218619120, 7055);
        assert_eq!(part1(INPUT), SOLUTION.0);
        assert_eq!(part2(INPUT), SOLUTION.1);
    }
}
