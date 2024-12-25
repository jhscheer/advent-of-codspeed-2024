use arrayvec::ArrayVec;

const COL: usize = 5;
const ROW: usize = 7;
const ARRAY: usize = 512;

const INIT: usize = 0b0001_0001_0001_0001_0001;
const MASK: usize = 0b1000_1000_1000_1000_1000;

type Input = str;
type Output = usize;

struct Parsed(ArrayVec<usize, ARRAY>, ArrayVec<usize, ARRAY>);

impl From<&Input> for Parsed {
    fn from(input: &Input) -> Self {
        let input = input.as_bytes();

        let mut keys = ArrayVec::<usize, ARRAY>::new();
        let mut locks = ArrayVec::<usize, ARRAY>::new();

        let mut offset = 0;
        while offset <= input.len() {
            let schematic = unsafe { &input.get_unchecked(offset..) };

            let mut buffer = INIT;
            for c in 0..COL {
                for r in 1..ROW - 1 {
                    let b = *unsafe { schematic.get_unchecked(r * (COL + 1) + c) };
                    buffer += (1 & usize::from(b == b'#')) << (c * 4);
                }
            }

            if *unsafe { schematic.get_unchecked(0) } == b'#' {
                locks.push(buffer);
            } else {
                keys.push(buffer);
            }

            offset += (COL + 1) * ROW + 1;
        }

        Self(keys, locks)
    }
}

#[aoc(day25, part1)]
pub fn part1(input: &Input) -> Output {
    let Parsed(keys, locks) = input.into();

    let mut fit = 0;
    for key in &keys {
        for lock in &locks {
            if (key + lock) & MASK == 0 {
                fit += 1;
            }
        }
    }

    fit
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../input/2024/day25.txt");
    const SOLUTION: Output = 2815;

    #[test]
    fn input() {
        assert_eq!(part1(INPUT), SOLUTION);
    }
}
