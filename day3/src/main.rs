use std::fs;

fn main() {
    let mut priority_sum: u32 = 0;

    let rucksack_string = fs::read_to_string("src/input.txt").expect("Couldn't read file");

    let _ruckasck_split: Vec<()> = rucksack_string
        .split_terminator("\n")
        .map(|rucksack| {
            let first_compartment_items = &rucksack[0..rucksack.len() / 2]; // Risky shieeet
            let second_compartment_items = &rucksack[rucksack.len() / 2..]; // Risky shieet
            let matching_item_option =
                get_matching_item_option(first_compartment_items, second_compartment_items);
            match matching_item_option {
                Some(item) => priority_sum += calculate_priority(item).unwrap(),
                None => {
                    println!("No matching item in rucksack");
                    return;
                }
            }
        })
        .collect();

    println!("{priority_sum}");
}

fn calculate_priority<'a>(matching_item: char) -> Result<u32, &'a str> {
    match matching_item {
        'a'..='z' => Ok(matching_item as u32 - 96),
        'A'..='Z' => Ok(matching_item as u32 - 38),
        _ => Err("Vomited error"),
    }
}

fn get_matching_item_option(
    first_compartment_items: &str,
    second_compartment_items: &str,
) -> Option<char> {
    for item in first_compartment_items.chars() {
        if match_in_other_compartment(item, second_compartment_items) {
            return Some(item);
        }
    }
    None
}

fn match_in_other_compartment(item_to_match: char, compartment_items: &str) -> bool {
    // check if item is a valid item
    if !item_to_match.is_ascii_alphabetic() {
        return false;
    }
    if compartment_items.contains(item_to_match) {
        return true;
    }
    false
}
