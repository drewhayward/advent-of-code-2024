use crate::solution::Solution;

#[derive(Debug, Clone)]
struct Occupied {
    size: usize,
    file_id: u64,
}

#[derive(Debug, Clone)]
struct Free {
    size: usize,
}

#[derive(Debug, Clone)]
enum DiskSection {
    Occupied(Occupied),
    Free(Free),
}

fn parse_input(puzzle_input: String) -> Vec<usize> {
    puzzle_input
        .trim()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect()
}

pub struct DiskFragmenter;

impl Solution for DiskFragmenter {
    fn part1(puzzle_input: String) -> String {
        let nums = parse_input(puzzle_input);

        // Build an array of the already compacted file blocks
        let mut blocks = Vec::new();
        for (i, file) in nums.iter().step_by(2).enumerate() {
            blocks.append(&mut vec![i; *file]);
        }

        let mut checksum = 0;
        let mut left = blocks.iter();
        let mut right = blocks.iter().rev();
        let mut iterations = 0;

        'outer: for (i, num) in nums.iter().enumerate() {
            for _ in 0..*num {
                if iterations >= blocks.len() {
                    break 'outer;
                }
                let next_block = match i % 2 {
                    // file
                    0 => left.next(),
                    // space
                    1 => right.next(),
                    _ => unreachable!(),
                };
                checksum += iterations * next_block.unwrap();
                iterations += 1;
            }
        }

        checksum.to_string()
    }

    fn part2(puzzle_input: String) -> String {
        let nums = parse_input(puzzle_input);

        // Build sections list
        let mut sections = Vec::new();
        let mut block_idx = 0;
        for (file_id, file_size) in nums.iter().enumerate() {
            sections.push(match file_id % 2 {
                0 => DiskSection::Occupied(Occupied {
                    size: *file_size,
                    file_id: (file_id / 2) as u64,
                }),
                1 => DiskSection::Free(Free { size: *file_size }),
                _ => unreachable!(),
            });
            block_idx += file_size;
        }

        let mut last_id = 0;
        if let Some(DiskSection::Occupied(occupied)) = sections.last() {
            last_id = occupied.file_id;
        }

        for id in (0..last_id + 1).rev() {
            // Fetch the current file section
            let (section_idx, section) = sections
                .iter()
                .enumerate()
                .filter_map(|(i, s)| match s {
                    DiskSection::Occupied(occupied) if occupied.file_id == id => {
                        Some((i, occupied))
                    }
                    _ => None,
                })
                .next()
                .expect("Current section id should exist");

            // Find the left-most free section which could hold our current file id
            let slot_tuple = sections
                .iter()
                .enumerate()
                .filter_map(|(i, s)| match s {
                    DiskSection::Free(slot) if slot.size >= section.size => Some((i, slot)),
                    _ => None,
                })
                .next();

            if let Some((slot_idx, slot)) = slot_tuple {
                if slot_idx > section_idx {
                    continue
                }
                // Remove the file section and empty slot and add it in correctly
                let section = section.clone();
                let slot = slot.clone();

                // First swap out the file for an empty section. We don't need to compact
                // these since we are always moving right to left
                let _ = std::mem::replace(&mut sections[section_idx], DiskSection::Free(Free { size: section.size }));

                if slot.size > section.size {
                    let new_sections = vec![
                        DiskSection::Occupied(Occupied {
                            size: section.size,
                            file_id: section.file_id,
                        }),
                        DiskSection::Free(Free {
                            size: slot.size - section.size,
                        }),
                    ];

                    sections = [&sections[0..slot_idx], &new_sections, &sections[slot_idx + 1..]].concat();
                } else {
                    let _ = std::mem::replace(&mut sections[slot_idx], DiskSection::Occupied(Occupied {
                        size: section.size,
                        file_id: section.file_id,
                    }));
                };
            }
        }

        
        sections.iter().fold((0, 0), |(block_idx, checksum), section| match section {
            DiskSection::Free(free) => (block_idx + free.size, checksum),
            DiskSection::Occupied(occupied) => {
                let mut c = checksum;
                for idx in (block_idx..block_idx+occupied.size) {
                    c += (idx as u64) * occupied.file_id;
                }
                (block_idx + occupied.size, c)
            },
        }).1.to_string()
    }
}
