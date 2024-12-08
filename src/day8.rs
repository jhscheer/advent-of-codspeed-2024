use core::ops::{Add, AddAssign, Neg, Sub};

type Input<'a> = &'a str;
type Output = usize;

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Coord(isize, isize);

impl Neg for Coord {
    type Output = Self;

    #[inline]
    fn neg(self) -> Self::Output {
        Self(self.0.wrapping_neg(), self.1.wrapping_neg())
    }
}

impl Add for Coord {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_add(rhs.0), self.1.wrapping_add(rhs.1))
    }
}

impl AddAssign for Coord {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = self.0.wrapping_add(rhs.0);
        self.1 = self.1.wrapping_add(rhs.1);
    }
}

impl Sub for Coord {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.wrapping_sub(rhs.0), self.1.wrapping_sub(rhs.1))
    }
}

struct Grid {
    antennas: [Vec<Coord>; 256], // HashMap
    bounds: Coord,               // Bounds of the map
    antinodes: Vec<Vec<bool>>,   // HashSet
}

impl Grid {
    fn contains(&self, Coord(r, c): &Coord) -> bool {
        (0..self.bounds.0).contains(r) && (0..self.bounds.1).contains(c)
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut antennas: [Vec<Coord>; 256] = core::array::from_fn(|_| Vec::with_capacity(16));

        let mut bounds = Coord(0, 0);
        let mut col_len = 0;

        for &ch in input.as_bytes() {
            if ch == b'\n' {
                bounds.0 += 1;
                col_len = bounds.1;
                bounds.1 = 0;
                continue;
            }
            if ch.is_ascii_alphanumeric() {
                antennas[ch as usize].push(bounds);
            }
            bounds.1 += 1;
        }
        bounds.1 = col_len;

        let antinodes = vec![vec![false; bounds.1 as usize]; bounds.0 as usize];

        Self {
            antennas,
            bounds,
            antinodes,
        }
    }
}

#[aoc(day8, part1)]
pub fn part1(input: Input) -> Output {
    let mut grid = Grid::from(input);
    for coords in &grid.antennas {
        for (i, a1) in coords.iter().enumerate() {
            for a2 in &coords[(i + 1)..] {
                let delta = *a2 - *a1;
                let antinode1 = *a1 + delta.neg();
                let antinode2 = *a2 + delta;

                if grid.contains(&antinode1) {
                    grid.antinodes[antinode1.0 as usize][antinode1.1 as usize] = true;
                }
                if grid.contains(&antinode2) {
                    grid.antinodes[antinode2.0 as usize][antinode2.1 as usize] = true;
                }
            }
        }
    }
    grid.antinodes.iter().flatten().filter(|&&x| x).count()
}

#[aoc(day8, part2)]
pub fn part2(input: Input) -> Output {
    let mut grid = Grid::from(input);
    for coords in &grid.antennas {
        for (i, a1) in coords.iter().enumerate() {
            for a2 in &coords[(i + 1)..] {
                let delta = *a2 - *a1;
                let mut antinode1 = *a1;
                let mut antinode2 = *a2;

                while grid.contains(&antinode1) {
                    grid.antinodes[antinode1.0 as usize][antinode1.1 as usize] = true;
                    antinode1 += delta.neg();
                }
                while grid.contains(&antinode2) {
                    grid.antinodes[antinode2.0 as usize][antinode2.1 as usize] = true;
                    antinode2 += delta;
                }
            }
        }
    }
    grid.antinodes.iter().flatten().filter(|&&x| x).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";
    const EXAMPLE2: &str = "T....#....
...T......
.T....#...
.........#
..#.......
..........
...#......
..........
....#.....
..........
";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output, Output) = (14, 34, 9);
        assert_eq!(part1(EXAMPLE1), EXPECTED.0);
        assert_eq!(part2(EXAMPLE1), EXPECTED.1);
        assert_eq!(part2(EXAMPLE2), EXPECTED.2);
    }

    const INPUT: &str = include_str!("../input/2024/day8.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (269, 949);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
