type Output = usize;
type Input = Vec<(usize, usize)>;

fn input_generator(input: &str, is_part2: bool) -> Input {
    let mut do_ranges = Vec::new();
    if is_part2 {
        enum Instructions {
            Do(usize),
            Dont(usize),
        }
        impl Instructions {
            fn key(&self) -> usize {
                match self {
                    Self::Do(n) => *n,
                    Self::Dont(n) => *n,
                }
            }
        }

        let mut dos = input
            .match_indices("do()")
            .map(|(n, _)| Instructions::Do(n))
            .collect::<Vec<_>>();
        let mut donts = input
            .match_indices("don't()")
            .map(|(n, _)| Instructions::Dont(n))
            .collect::<Vec<_>>();

        dos.append(&mut donts);
        dos.push(Instructions::Dont(input.len()));
        dos.sort_by_key(|x| x.key());

        let mut do_range_start = 0;
        let mut inside_dont_range = false;
        for instruction in dos {
            match instruction {
                Instructions::Dont(end) if !inside_dont_range => {
                    do_ranges.push(std::ops::Range {
                        start: do_range_start,
                        end,
                    });
                    inside_dont_range = true;
                }
                Instructions::Do(start) if inside_dont_range => {
                    do_range_start = start;
                    inside_dont_range = false;
                }
                _ => {}
            }
        }
    }

    let mut vec_operands = Vec::new();
    let mut cursor = 0;
    while let Some(offset_mul_start) = input[cursor..].find("mul(") {
        cursor = cursor + offset_mul_start + 4;

        if is_part2 {
            let mut is_in_do_range = false;
            for range in &do_ranges {
                if range.contains(&cursor) {
                    is_in_do_range = true;
                    break;
                }
            }
            if !is_in_do_range {
                continue;
            }
        }

        let offset_mul_end = match input[cursor..].find(")") {
            Some(n) => n + cursor,
            None => break,
        };

        let operands = match input[cursor..offset_mul_end].split_once(",") {
            Some(n) => n,
            None => continue,
        };

        let operands_parsed = (
            match operands.0.parse::<usize>() {
                Ok(n) => n,
                _ => continue,
            },
            match operands.1.parse::<usize>() {
                Ok(n) => n,
                _ => continue,
            },
        );

        vec_operands.push(operands_parsed);
        cursor = offset_mul_end + 1;
    }

    vec_operands
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> Output {
    input_generator(input, false)
        .iter()
        .map(|&(a, b)| a * b)
        .sum()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> Output {
    input_generator(input, true)
        .iter()
        .map(|&(a, b)| a * b)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    const EXPECTED: (Output, Output) = (161, 48);

    #[test]
    fn example1() {
        assert_eq!(part1(EXAMPLE1), EXPECTED.0);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(EXAMPLE2), EXPECTED.1);
    }
}
