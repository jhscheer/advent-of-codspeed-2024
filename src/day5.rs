use std::collections::HashMap;

type Output = usize;
type Input = (Vec<(usize, usize)>, Vec<Vec<usize>>);

fn input_generator(input: &str) -> Input {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|r| r.split_once("|").unwrap())
        .map(|(a, b)| (a.parse().unwrap(), b.parse().unwrap()))
        .collect::<Vec<_>>();
    let updates = updates
        .lines()
        .map(|l| l.split(",").map(|s| s.parse().unwrap()).collect())
        .collect();
    (rules, updates)
}

#[aoc(day5, part1)]
pub fn part1(input: &str) -> Output {
    let input = input_generator(input);
    let mut rules = HashMap::new();
    for rule in &input.0 {
        let entry = rules.entry(rule.0).or_insert(vec![rule.1]);
        entry.push(rule.1);
    }

    let mut sum = 0;
    'outer: for update in &input.1 {
        for (i, page) in update.iter().enumerate() {
            if let Some(after) = rules.get(page) {
                if update.iter().take(i).any(|x| after.contains(x)) {
                    continue 'outer;
                }
            }
        }
        sum += update.get(update.len() / 2).unwrap();
    }
    sum
}

#[aoc(day5, part2)]
pub fn part2(input: &str) -> Output {
    let input = input_generator(input);
    let mut rules = HashMap::new();
    for rule in &input.0 {
        let entry = rules.entry(rule.0).or_insert(vec![rule.1]);
        entry.push(rule.1);
    }

    let mut sum = 0;
    'update: for update in &input.1 {
        for (i, page) in update.iter().enumerate() {
            if let Some(after) = rules.get(page) {
                if update.iter().take(i).any(|x| after.contains(x)) {
                    let mut ordered_pages = Vec::with_capacity(update.len());

                    while ordered_pages.len() < update.len() {
                        'inner: for current_page in update {
                            if ordered_pages.contains(&current_page) {
                                continue;
                            }
                            for other_page in update.iter().filter(|&x| x != current_page) {
                                if ordered_pages.contains(&other_page) {
                                    continue;
                                }
                                if let Some(after) = rules.get(other_page) {
                                    if after.contains(current_page) {
                                        continue 'inner;
                                    }
                                }
                            }
                            ordered_pages.push(current_page);
                            if ordered_pages.len() > update.len() / 2 {
                                sum += *ordered_pages.last().unwrap();
                                continue 'update;
                            }
                        }
                    }
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    const EXPECTED: (Output, Output) = (143, 123);

    #[test]
    fn example1() {
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }
}
