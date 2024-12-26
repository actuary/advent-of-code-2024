use std::collections::VecDeque;

use aoc2024::triangular;

#[derive(Copy, Clone, Debug)]
enum Block {
    File(u8, u64),
    Free(u8),
}

fn parse(data: &str) -> VecDeque<Block> {
    let mut dequeue = VecDeque::new();
    data.trim()
        .chars()
        .map(|ch| ch.to_string().parse().unwrap())
        .enumerate()
        .for_each(|(i, digit)| {
            if digit == 0 {
            } else if i % 2 == 0 {
                dequeue.push_back(Block::File(digit, i as u64 / 2));
            } else {
                dequeue.push_back(Block::Free(digit));
            }
        });

    dequeue
}

fn checksum(filesystem: &VecDeque<Block>) -> u64 {
    let mut result: u64 = 0;

    let mut idx: u64 = 0;
    for block in filesystem {
        match block {
            Block::Free(size) => {
                idx += *size as u64;
            }
            Block::File(size, id) => {
                result += triangular(idx, idx + *size as u64 - 1) * id;
                idx += *size as u64;
            }
        }
    }
    result
}

pub fn part1(data: &str) -> u64 {
    let mut filesystem = parse(data);

    let mut i: usize = 0;

    while i < filesystem.len() - 1 {
        let block = filesystem.remove(i).unwrap();
        match block {
            Block::File(_, _) => {
                // just put it back...
                filesystem.insert(i, block);
                i += 1;
            }
            Block::Free(free_size) => {
                let end_block = filesystem.pop_back().unwrap();

                match end_block {
                    Block::Free(_) => {
                        // put back the block we took,
                        // ignore the free block we popped - leave it popped...
                        filesystem.insert(i, block);
                        // continue on and try again (don't increment i).
                    }
                    Block::File(end_size, id) => {
                        if end_size <= free_size {
                            // then use up some of that free space
                            filesystem.insert(i, Block::File(end_size, id));
                        }

                        if end_size < free_size {
                            // if there's space left over, then it's free space
                            filesystem.insert(i + 1, Block::Free(free_size - end_size));
                        }

                        if end_size > free_size {
                            // if we can't fit it, just fit as much as we can,
                            // and put the rest at the nd
                            filesystem.insert(i, Block::File(free_size, id));
                            filesystem.push_back(Block::File(end_size - free_size, id));
                        }
                        i += 1;
                    }
                }
            }
        }
    }

    checksum(&filesystem)
}

pub fn part2(data: &str) -> u64 {
    // it's pretty clear at this point that i need to study some rust, as
    // I don't know what I'm doing.
    // I'm copying everything when I don't understand how to do something, as a
    // result this is horrendously slow.

    let mut filesystem = parse(data);

    let last_file_id: u64 = *filesystem
        .iter()
        .filter_map(|block| match block {
            Block::File(_, id) => Some(id),
            Block::Free(_) => None,
        })
        .last()
        .unwrap();

    for disk_id in (0..=last_file_id).rev() {
        let pos: usize = filesystem
            .iter()
            .enumerate()
            .filter(|(_, &block)| match block {
                Block::File(_, id) => id == disk_id,
                _ => false,
            })
            .next()
            .unwrap()
            .0;

        let block = filesystem.remove(pos).unwrap();
        let mut moved: bool = false;
        match block {
            Block::File(size, _) => {
                for j in 0..pos {
                    let free_block = filesystem.remove(j).unwrap();
                    match free_block {
                        Block::File(_, _) => {
                            filesystem.insert(j, free_block);
                        }
                        Block::Free(free_size) => {
                            if size < free_size {
                                filesystem.insert(j, block);
                                filesystem.insert(pos, Block::Free(size));
                                filesystem.insert(j + 1, Block::Free(free_size - size));
                                moved = true;
                                break;
                            }

                            if size == free_size {
                                filesystem.insert(j, block);
                                filesystem.insert(pos, Block::Free(size));
                                moved = true;
                                break;
                            }

                            if size > free_size {
                                filesystem.insert(j, free_block);
                            }
                        }
                    }
                }

                if !moved {
                    filesystem.insert(pos, block);
                }
            }
            Block::Free(_) => {
                panic!("Should never happen");
            }
        }
    }

    checksum(&filesystem)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_works() {
        let data = "2333133121414131402";
        assert_eq!(part1(data), 1928);
    }

    #[test]
    fn part2_works() {
        let data = "2333133121414131402";
        assert_eq!(part2(data), 2858);
    }
}
