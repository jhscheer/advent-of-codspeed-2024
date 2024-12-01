type Input = (Vec<usize>, Vec<usize>);
type Output = usize;

fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(a, b)| (a.parse::<usize>().unwrap(), b.parse::<usize>().unwrap()))
        .unzip()
}

#[aoc(day1, part1)]
pub fn part1(input: &str) -> Output {
    let (mut v1, mut v2) = input_generator(input);
    v1.sort_unstable();
    v2.sort_unstable();

    v1.iter().zip(v2.iter()).map(|(&a, &b)| a.abs_diff(b)).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> Output {
    let mut sim_scores = std::collections::HashMap::new();
    let (v1, v2) = input_generator(input);

    v1.iter()
        .map(|&n| {
            let count = *sim_scores.entry(n).or_insert_with(|| {
                // count how often number from vec1 appears in vec2
                v2.iter()
                    .fold(0, |acc, &x| if x == n { acc + 1 } else { acc })
            });
            n * count
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
    const EXPECTED: (Output, Output) = (11, 31);

    #[test]
    fn example1() {
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }
}
