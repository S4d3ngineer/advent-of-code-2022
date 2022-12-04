use std::fs;

fn main() {
    let mut priority_sum: u32 = 0;
    let mut priority_badges_sum: u32 = 0;

    let rucksack_string = fs::read_to_string("src/input.txt").expect("Couldn't read file");

    let _ruckasck_split: Vec<()> = rucksack_string
        .lines()
        .map(|rucksack| {
            let first_compartment_items = &rucksack[0..rucksack.len() / 2]; // Risky shieeet
            let second_compartment_items = &rucksack[rucksack.len() / 2..]; // Risky shieet
            let matching_item_option =
                get_matching_item_option([first_compartment_items, second_compartment_items].to_vec());
            match matching_item_option {
                Some(item) => priority_sum += calculate_priority(item).unwrap(),
                None => {
                    println!("No matching item in rucksack");
                    return;
                }
            }
        })
        .collect();

    let rucksacks_vector: Vec<&str> = rucksack_string
        .lines()
        .map(|rucksack| rucksack)
        .collect();


    for i in (0..rucksacks_vector.len()).step_by(3) {
        let matching_item_option = get_matching_item_option(rucksacks_vector[i..i+3].to_vec());
        match matching_item_option {
            Some(item) => priority_badges_sum += calculate_priority(item).unwrap(),
            None => {
                println!("No matching badges in rucksacks {i} to {:?}", i + 2);
                return;
            }
        }

    }

    println!("Priority: {priority_sum}");
    println!("Badges priority: {priority_badges_sum}");
}

fn calculate_priority<'a>(matching_item: char) -> Result<u32, &'a str> {
    match matching_item {
        'a'..='z' => Ok(matching_item as u32 - 96),
        'A'..='Z' => Ok(matching_item as u32 - 38),
        _ => Err("Vomited error"),
    }
}


fn get_matching_item_option(
    sets_to_match: Vec<&str>,
) -> Option<char> {
    for item in sets_to_match[0].chars() {
        if match_item_in_all_sets(item, sets_to_match[1..].to_vec()) {
            return Some(item);
        }
    }

    None
}

fn match_item_in_all_sets(item_to_match: char, sets_to_match: Vec<&str>) -> bool {
    if !item_to_match.is_ascii_alphabetic() {
        return false;
    } // unnecessary operation to perform every time - change later
    if sets_to_match[0].contains(item_to_match) {
        if sets_to_match[1..].to_vec().len() >= 1 {
            match_item_in_all_sets(item_to_match, sets_to_match[1..].to_vec())
        } else {
            true
        }
    } else {
        false
    }
} 

