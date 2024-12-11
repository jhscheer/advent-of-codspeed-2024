use core::ops::Add;
use fxhash::FxHashSet;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point(isize, isize);

impl From<(usize, usize)> for Point {
    fn from((r, c): (usize, usize)) -> Self {
        Self(r as isize, c as isize)
    }
}

impl From<(i32, i32)> for Point {
    fn from((r, c): (i32, i32)) -> Self {
        Self(r as isize, c as isize)
    }
}

impl Add for Point {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        unsafe { Self(self.0.unchecked_add(rhs.0), self.1.unchecked_add(rhs.1)) }
    }
}

const MAX_LEN: usize = 64;
const DEFAULT_CAPACITY: usize = 64;

struct Map {
    inner: [[u8; MAX_LEN]; MAX_LEN],
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let mut inner = [[0u8; MAX_LEN]; MAX_LEN];

        let (mut row, mut col) = (0, 0);
        for b in value.as_bytes().iter() {
            if *b == 10 {
                // 10: '\n'
                row += 1;
                col = 0;
                continue;
            }
            unsafe {
                let entry = inner.get_unchecked_mut(row).get_unchecked_mut(col);
                *entry = *b;
            }
            col += 1;
        }
        Self { inner }
    }
}

impl Map {
    unsafe fn get_unchecked(&self, Point(r, c): Point) -> u8 {
        *self
            .inner
            .get_unchecked(r as usize)
            .get_unchecked(c as usize)
    }

    unsafe fn contains(&self, Point(r, c): Point) -> bool {
        (0..(self.inner.len() as isize)).contains(&r)
            && (0..(self.inner.get_unchecked(0).len() as isize)).contains(&c)
    }

    fn adj4_points(&self, rhs: Point) -> [Option<Point>; 4] {
        let mut neighbors = [None; 4];
        for (i, &direction) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().enumerate() {
            let neighbor = rhs + direction.into();
            neighbors[i] = if unsafe { self.contains(neighbor) } {
                Some(neighbor)
            } else {
                None
            };
        }
        neighbors
    }
}

fn parse_input(input: &str) -> Map {
    input.into()
}

unsafe fn dfs(point: Point, map: &Map, p1: &mut Option<FxHashSet<Point>>) -> usize {
    if let Some(seen) = p1 {
        if !seen.insert(point) {
            return 0;
        }
    }

    if map.get_unchecked(point) == 57 {
        // 57: '9'
        return 1;
    }

    let mut result = 0;
    for adj_point in map.adj4_points(point).into_iter().flatten() {
        if map.get_unchecked(adj_point) == 1 + map.get_unchecked(point) {
            result += dfs(adj_point, map, p1);
        }
    }
    result
}

fn solve(map: &Map, p1: bool) -> usize {
    let grid = &map.inner;
    let mut sum = 0;

    for (r, row) in grid.iter().enumerate() {
        for (c, &height) in row.iter().enumerate() {
            if height == 48 {
                // 48: '0'
                let mut seen = if p1 {
                    Some(FxHashSet::with_capacity_and_hasher(
                        DEFAULT_CAPACITY,
                        Default::default(),
                    ))
                } else {
                    None
                };
                sum += unsafe { dfs((r, c).into(), map, &mut seen) };
            }
        }
    }

    sum
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    solve(&parse_input(input), true)
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    solve(&parse_input(input), false)
}
#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "89010123\n78121874\n87430965\n96549874\n45678903\n32019012\n01329801\n10456732\n";

    #[test]
    fn xmpls() {
        const EXPECTED: (usize, usize) = (36, 81);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day10.txt");

    #[test]
    fn input() {
        const SOLUTION: (usize, usize) = (776, 1657);
        assert_eq!(part1(INPUT), SOLUTION.0);
        assert_eq!(part2(INPUT), SOLUTION.1);
    }
}
