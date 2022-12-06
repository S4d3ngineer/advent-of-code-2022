use std::fs;

#[derive(Debug)]
struct Range {
    min: u32,
    max: u32,
}

fn main() {
    let puzzle_input_path = "src/input.txt";

    let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();

    let elves_pairs_vector = puzzle_input
        .lines()
        .map(|line| {
            line.split_terminator(",")
                .map(|range| range.split_terminator('-').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!(
        "Contained ranges count: {:?}",
        solve_part_1(&elves_pairs_vector)
    );
    println!(
        "Contained ranges count: {:?}",
        solve_part_2(&elves_pairs_vector)
    );
}

fn solve_part_1(elves_pairs_vector: &Vec<Vec<Vec<&str>>>) -> u32 {
    let mut contained_count = 0;

    for pair in elves_pairs_vector {
        if pair.len() != 2 {
            panic!(
                "Incorrect pair: {:?}. Pair should consist of two elements.",
                pair
            );
        }

        let first_elf = &pair[0];
        if first_elf.len() != 2 {
            panic!("Incorrect range for one of elfs in {:?}", pair)
        }
        let second_elf = &pair[1];
        if second_elf.len() != 2 {
            panic!("Incorrect range for one of elfs in {:?}", pair)
        }

        let first_elf_range = Range {
            min: first_elf[0].parse().unwrap(),
            max: first_elf[1].parse().unwrap(),
        };
        let second_elf_range = Range {
            min: second_elf[0].parse().unwrap(),
            max: second_elf[1].parse().unwrap(),
        };

        if are_ranges_contained(first_elf_range, second_elf_range) {
            contained_count += 1;
        }
    }

    contained_count
}

fn are_ranges_contained(first_range: Range, second_range: Range) -> bool {
    if (first_range.min >= second_range.min && first_range.max <= second_range.max)
        || (second_range.min >= first_range.min && second_range.max <= first_range.max)
    {
        return true;
    }

    false
}

fn solve_part_2(elves_pairs_vector: &Vec<Vec<Vec<&str>>>) -> u32 {
    let mut overlapping_count = 0;

    for pair in elves_pairs_vector {
        if pair.len() != 2 {
            panic!(
                "Incorrect pair: {:?}. Pair should consist of two elements.",
                pair
            );
        }

        let first_elf = &pair[0];
        if first_elf.len() != 2 {
            panic!("Incorrect range for one of elfs in {:?}", pair)
        }
        let second_elf = &pair[1];
        if second_elf.len() != 2 {
            panic!("Incorrect range for one of elfs in {:?}", pair)
        }

        let first_elf_range = Range {
            min: first_elf[0].parse().unwrap(),
            max: first_elf[1].parse().unwrap(),
        };
        let second_elf_range = Range {
            min: second_elf[0].parse().unwrap(),
            max: second_elf[1].parse().unwrap(),
        };

        if does_ranges_overlap(first_elf_range, second_elf_range) {
            overlapping_count += 1;
        }
    }

    overlapping_count
}

fn does_ranges_overlap(first_range: Range, second_range: Range) -> bool {
    if (first_range.min >= second_range.min && first_range.min <= second_range.max)
        || (second_range.min >= first_range.min && second_range.min <= first_range.max)
    {
        return true;
    }

    false
}
