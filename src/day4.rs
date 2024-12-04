use core::str::from_utf8_unchecked;

type Output = usize;
type Input = Vec<Vec<u8>>;

#[aoc(day4, part1)]
pub fn part1(input: &str) -> Output {
    let mut input: Input = input.lines().map(|l| l.as_bytes().to_vec()).collect();

    const PATTERN: &str = "XMAS";
    let mut sum = 0;

    let row_len = input.len();
    let col_len = unsafe { input.get_unchecked(0) }.len();

    let mut transpose: Input = (0..col_len)
        .map(|i| {
            input
                .iter()
                .map(|inner| unsafe { *inner.get_unchecked(i) })
                .collect::<Vec<_>>()
        })
        .collect();

    // matrix rows left->right:
    sum += input
        .iter()
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // transpose rows left->right:
    sum += transpose
        .iter()
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // diagonals t-left->b-right:
    let mut diagonals: Input = vec![vec![]; row_len + col_len - 1];
    for i in 0..col_len {
        for j in 0..row_len {
            diagonals[i + j].push(unsafe { *input.get_unchecked(j).get_unchecked(i) });
        }
    }
    sum += diagonals
        .iter()
        .filter(|l| l.len() >= 4)
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // mirror matrix:
    for row in input.iter_mut() {
        row.reverse();
    }

    // matrix rows right->left:
    sum += input
        .iter()
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // mirror transpose:
    for row in transpose.iter_mut() {
        row.reverse();
    }
    // transpose rows right->left:
    sum += transpose
        .iter()
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // mirror diagonals:
    for row in diagonals.iter_mut() {
        row.reverse();
    }
    // diagonals t-left->b-right (rev):
    sum += diagonals
        .iter()
        .filter(|l| l.len() >= 4)
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // diagonals t-right->b-left:
    let mut diagonals: Input = vec![vec![]; row_len + col_len - 1];
    for i in 0..col_len {
        for j in 0..row_len {
            unsafe {
                diagonals
                    .get_unchecked_mut(i + j)
                    .push(*input.get_unchecked(j).get_unchecked(i))
            };
        }
    }
    sum += diagonals
        .iter()
        .filter(|l| l.len() >= 4)
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    // mirror diagonals:
    for row in diagonals.iter_mut() {
        row.reverse();
    }
    // diagonals t-right->b-left (rev):
    sum += diagonals
        .iter()
        .filter(|l| l.len() >= 4)
        .map(|v| unsafe { from_utf8_unchecked(v) }.matches(PATTERN).count())
        .sum::<usize>();

    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &str) -> Output {
    let row_len = input.lines().count();
    let col_len = unsafe { input.find('\n').unwrap_unchecked() + 1 }; // +1 because '\n'
    let input = input.as_bytes();

    let mut sum = 0;
    let mut cross: [u8; 4];
    for r in 1..(row_len - 1) {
        for c in 1..(col_len - 2) {
            let pos = r * col_len + c;
            if *unsafe { input.get_unchecked(pos) } == b'A' {
                cross = [
                    unsafe { *input.get_unchecked(pos - col_len - 1) },
                    unsafe { *input.get_unchecked(pos - col_len + 1) },
                    unsafe { *input.get_unchecked(pos + col_len - 1) },
                    unsafe { *input.get_unchecked(pos + col_len + 1) },
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
    fn example1() {
        assert_eq!(part1(EXAMPLE), 18);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(EXAMPLE), 9);
    }
}
