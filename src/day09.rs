type Input<'a> = &'a str;
type Output = usize;

#[derive(Clone, PartialEq)]
enum Slot {
    Empty,
    File(usize),
}

#[derive(Clone, PartialEq)]
struct Block {
    start: usize,
    size: usize,
    slot: Slot,
}

fn parse_input(input: &str) -> (Vec<Block>, Vec<Slot>) {
    let mut bytes = input.as_bytes().iter();
    let mut id = 0;
    let mut index = 0;
    let mut blocks = Vec::new();
    let mut slots = Vec::new();

    while let Some(filled) = bytes.next().and_then(|b| b.checked_sub(b'0')) {
        blocks.push(Block {
            start: index,
            size: filled as usize,
            slot: Slot::File(id),
        });
        for _ in 0..filled {
            slots.push(Slot::File(id));
        }

        index += filled as usize;

        if let Some(whitespace) = bytes.next().and_then(|b| b.checked_sub(b'0')) {
            blocks.push(Block {
                start: index,
                size: whitespace as usize,
                slot: Slot::Empty,
            });
            for _ in 0..whitespace {
                slots.push(Slot::Empty);
            }
            index += whitespace as usize;
        }

        id += 1;
    }

    (blocks, slots)
}

#[aoc(day9, part1)]
pub fn part1(input: Input) -> Output {
    let (_blocks, slots) = parse_input(input);
    let mut reversed = slots
        .iter()
        .rev()
        .enumerate()
        .filter(|(_, slot)| matches!(slot, Slot::File(_)));

    let mut checksum = 0;
    let mut high_j = 0;
    for (i, slot) in slots.iter().enumerate() {
        match slot {
            Slot::Empty => {
                if let Some((j, revered_block)) = reversed.next() {
                    high_j = j + 1;
                    if j >= slots.len() - i {
                        break;
                    }

                    if let Slot::File(id) = revered_block {
                        checksum += i * *id;
                    }
                }
            }
            Slot::File(id) => {
                if high_j >= slots.len() - i {
                    break;
                }
                checksum += *id * i;
            }
        }
    }

    checksum
}

#[aoc(day9, part2)]
pub fn part2(input: Input) -> Output {
    let (blocks, mut slots) = parse_input(input);
    let mut free: Vec<Block> = blocks
        .iter()
        .filter(|b| matches!(b.slot, Slot::Empty))
        .cloned()
        .collect();

    for block in blocks.iter().rev() {
        if let Slot::File(id) = block.slot {
            if let Some((free_index, free_block)) = free
                .iter_mut()
                .enumerate()
                .find(|(_, b)| b.size >= block.size)
            {
                if free_block.start > block.start {
                    continue;
                }

                let new_slice = &mut slots[free_block.start..free_block.start + block.size];
                new_slice.fill(Slot::File(id));

                let old_slice = &mut slots[block.start..block.start + block.size];
                old_slice.fill(Slot::Empty);

                if free_block.size > block.size {
                    free_block.size -= block.size;
                    free_block.start += block.size;
                } else {
                    free.remove(free_index);
                }
            }
        }
    }

    slots
        .iter()
        .enumerate()
        .map(|(i, slot)| {
            if let Slot::File(id) = slot {
                i * *id
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "2333133121414131402\n";

    #[test]
    fn xmpls() {
        const EXPECTED: (Output, Output) = (1928, 2858);
        assert_eq!(part1(EXAMPLE), EXPECTED.0);
        assert_eq!(part2(EXAMPLE), EXPECTED.1);
    }

    const INPUT: &str = include_str!("../input/2024/day9.txt");

    #[test]
    fn input() {
        const EXPECTED: (Output, Output) = (6201130364722, 6221662795602);
        assert_eq!(part1(INPUT), EXPECTED.0);
        assert_eq!(part2(INPUT), EXPECTED.1);
    }
}
