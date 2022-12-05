use std::fs;

fn main() {
    let puzzle_input_path = "src/input.txt";

    let puzzle_input = fs::read_to_string(puzzle_input_path).unwrap();

    println!("{:?}", solve_part_1(puzzle_input));
}

fn solve_part_1(puzzle_input: String) -> u32 {
    let elves_pairs_vector = puzzle_input
        .lines()
        .map(|line| {
            line.split_terminator(",")
                .map(|range| range.split_terminator('-').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut contained_count = 0;

    for pair in elves_pairs_vector {
        if pair.len() != 2 {
            panic!(
                "Incorrect pair: {:?}. Pair should consist of two elements.",
                pair
            );
        }

        println!("Pair: {:?}", pair);

        let first_elf = &pair[0];
        if first_elf.len() != 2 {panic!("Incorrect range for one of elfs in {:?}", pair)}
        let second_elf = &pair[1];
        if second_elf.len() != 2 {panic!("Incorrect range for one of elfs in {:?}", pair)}

        let first_elf_range: (u32, u32) =
            (first_elf[0].parse().unwrap(), first_elf[1].parse().unwrap());
        let second_elf_range: (u32, u32) = (
            second_elf[0].parse().unwrap(),
            second_elf[1].parse().unwrap(),
        );

        if (first_elf_range.0 >= second_elf_range.0 && first_elf_range.1 <= second_elf_range.1)
            || (second_elf_range.0 >= first_elf_range.0 && second_elf_range.1 <= first_elf_range.1)
        {
            contained_count += 1;
            println!("Contained count: {:?}", contained_count);
            println!("Contained pair: {:?}", pair);
        }

    }

    contained_count
}
