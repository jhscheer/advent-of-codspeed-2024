type Output = usize;
type Input = Vec<Vec<usize>>;

fn parse_input(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_report_safe(report: &[usize]) -> bool {
    report.is_sorted_by(|a, b| a < b && b - a <= 3)
        || report.is_sorted_by(|a, b| a > b && a - b <= 3)
}

#[aoc(day2, part1)]
pub fn part1(input: &str) -> Output {
    let input = parse_input(input);
    input.iter().filter(|x| is_report_safe(x)).count()
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> Output {
    let input = parse_input(input);
    let mut count = 0;
    for report in input {
        if is_report_safe(&report) {
            count += 1;
        } else {
            for i in 0..report.len() {
                let mut tmp = report.to_owned();
                tmp.remove(i);
                if is_report_safe(&tmp) {
                    count += 1;
                    break;
                }
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output) = (2, 4);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day2.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (402, 455);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
