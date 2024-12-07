type Output = usize;
type Input<'a> = (usize, usize, &'a [u8]);

fn parse_input(input: &str) -> Input {
    let row_len = input.lines().count();
    let col_len = unsafe { input.find('\n').unwrap_unchecked() + 1 }; // +1 because '\n'
    let input = input.as_bytes();
    (row_len, col_len, input)
}

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Output {
    let (row_len, col_len, grid) = parse_input(input);
    let mut sum = 0;

    for r in 0..row_len {
        for c in 0..col_len {
            let pos = r * col_len + c;

            if c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'X'
                && unsafe { *grid.get_unchecked(pos + 1) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 2) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 3) } == b'S'
            {
                sum += 1;
            }
            if r + 3 < row_len
                && unsafe { *grid.get_unchecked(pos) } == b'X'
                && unsafe { *grid.get_unchecked(pos + col_len) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 2 * col_len) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 3 * col_len) } == b'S'
            {
                sum += 1;
            }
            if r + 3 < row_len
                && c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'X'
                && unsafe { *grid.get_unchecked(pos + col_len + 1) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 2 * col_len + 2) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 3 * col_len + 3) } == b'S'
            {
                sum += 1;
            }
            if c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'S'
                && unsafe { *grid.get_unchecked(pos + 1) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 2) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 3) } == b'X'
            {
                sum += 1;
            }
            if r + 3 < row_len
                && unsafe { *grid.get_unchecked(pos) } == b'S'
                && unsafe { *grid.get_unchecked(pos + col_len) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 2 * col_len) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 3 * col_len) } == b'X'
            {
                sum += 1;
            }
            if r + 3 < row_len
                && c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'S'
                && unsafe { *grid.get_unchecked(pos + col_len + 1) } == b'A'
                && unsafe { *grid.get_unchecked(pos + 2 * col_len + 2) } == b'M'
                && unsafe { *grid.get_unchecked(pos + 3 * col_len + 3) } == b'X'
            {
                sum += 1;
            }
            if r as isize - 3 >= 0
                && c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'S'
                && unsafe { *grid.get_unchecked(pos - col_len + 1) } == b'A'
                && unsafe { *grid.get_unchecked(pos - 2 * col_len + 2) } == b'M'
                && unsafe { *grid.get_unchecked(pos - 3 * col_len + 3) } == b'X'
            {
                sum += 1;
            }
            if r as isize - 3 >= 0
                && c + 3 < col_len
                && unsafe { *grid.get_unchecked(pos) } == b'X'
                && unsafe { *grid.get_unchecked(pos - col_len + 1) } == b'M'
                && unsafe { *grid.get_unchecked(pos - 2 * col_len + 2) } == b'A'
                && unsafe { *grid.get_unchecked(pos - 3 * col_len + 3) } == b'S'
            {
                sum += 1;
            }
        }
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> Output {
    let (row_len, col_len, grid) = parse_input(input);

    let mut sum = 0;
    let mut cross: [u8; 4];
    for r in 1..(row_len - 1) {
        for c in 1..(col_len - 2) {
            let pos = r * col_len + c;
            if *unsafe { grid.get_unchecked(pos) } == b'A' {
                cross = [
                    unsafe { *grid.get_unchecked(pos - col_len - 1) },
                    unsafe { *grid.get_unchecked(pos - col_len + 1) },
                    unsafe { *grid.get_unchecked(pos + col_len - 1) },
                    unsafe { *grid.get_unchecked(pos + col_len + 1) },
                ];

                match cross {
                    [b'M', b'M', b'S', b'S'] => sum += 1,
                    [b'S', b'S', b'M', b'M'] => sum += 1,
                    [b'S', b'M', b'S', b'M'] => sum += 1,
                    [b'M', b'S', b'M', b'S'] => sum += 1,
                    _ => {}
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output) = (18, 9);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day4.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (2543, 1930);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
