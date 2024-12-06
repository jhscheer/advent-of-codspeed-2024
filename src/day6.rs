struct Grid {
    map: Vec<u8>,
    row_len: usize,
    col_len: usize,
    start: (isize, isize),
}

impl Grid {
    fn find_start(&mut self) {
        for r in 0..self.row_len {
            for c in 0..self.col_len {
                let pos = r * self.col_len + c;
                if unsafe { *self.map.get_unchecked(pos) } == b'^' {
                    self.start = (c as isize, r as isize);
                    break;
                }
            }
        }
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let lines = value.lines().collect::<Vec<_>>();
        let map = lines
            .iter()
            .flat_map(|l| l.as_bytes())
            .copied()
            .collect::<Vec<_>>();
        let row_len = lines.len();
        let col_len = unsafe { lines.get_unchecked(0).len() };
        let mut grid = Self {
            map,
            row_len,
            col_len,
            start: (0, 0),
        };
        grid.find_start();
        grid
    }
}
const DIRECTION: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn detect_loop(
    grid: &Grid,
    mut path: Option<&mut Vec<usize>>,
    prev_state: &mut [u8],
    visited: &mut [bool],
) -> bool {
    prev_state.fill(0);
    let mut start = grid.start;

    if path.is_some() {
        visited.fill(false);
    }

    let mut direction = 0;
    loop {
        let pos = start.1 as usize * grid.col_len + start.0 as usize;
        if let Some(ref mut path) = path {
            if !unsafe { *visited.get_unchecked(pos) } {
                path.push(pos);
                unsafe {
                    let elem = visited.get_unchecked_mut(pos);
                    *elem = true;
                }
            }
        }

        let next_pos = (
            start.0 + unsafe { DIRECTION.get_unchecked(direction).0 },
            start.1 + unsafe { DIRECTION.get_unchecked(direction).1 },
        );
        if next_pos.0 < 0
            || next_pos.1 < 0
            || next_pos.0 >= grid.col_len as isize
            || next_pos.1 >= grid.row_len as isize
        {
            break;
        }

        if unsafe {
            *grid
                .map
                .get_unchecked(next_pos.1 as usize * grid.col_len + next_pos.0 as usize)
        } == b'#'
        {
            direction = (direction + 1) % 4;

            let mask = 1u8 << direction;
            if unsafe { prev_state.get_unchecked(pos) } & mask > 0 {
                return true;
            }
            unsafe {
                let elem = prev_state.get_unchecked_mut(pos);
                *elem |= mask;
            }
        } else {
            start = next_pos;
        }
    }
    false
}

type Input<'a> = &'a str;
type Output = usize;

#[aoc(day6, part1)]
pub fn part1(input: Input) -> Output {
    let grid = Grid::from(input);

    let mut prev_state = vec![0u8; grid.map.len()];
    let mut visited = vec![false; grid.map.len()];

    let mut path = Vec::new();
    detect_loop(&grid, Some(&mut path), &mut prev_state, &mut visited);
    path.len()
}

#[aoc(day6, part2)]
pub fn part2(input: Input) -> Output {
    let mut grid = Grid::from(input);

    let mut prev_state = vec![0u8; grid.map.len()];
    let mut visited = vec![false; grid.map.len()];

    let mut path = Vec::new();
    detect_loop(&grid, Some(&mut path), &mut prev_state, &mut visited);

    let mut loop_cnt = 0;
    for &i in &path[1..] {
        if unsafe { *grid.map.get_unchecked(i) } == b'.' {
            unsafe {
                let elem = grid.map.get_unchecked_mut(i);
                *elem = b'#';
            }

            if detect_loop(&grid, None, &mut prev_state, &mut visited) {
                loop_cnt += 1;
            }
            unsafe {
                let elem = grid.map.get_unchecked_mut(i);
                *elem = b'.';
            }
        }
    }
    loop_cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    const EXPECTED: (Output, Output) = (41, 6);

    #[test]
    fn example1() {
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
    }

    #[test]
    fn example2() {
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }
}
