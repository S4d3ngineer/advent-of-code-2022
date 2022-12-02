use std::fs;

fn main() {
    let file_path = "src/input.txt";

    let contents = fs::read_to_string(file_path).expect("Couldn't read the file");

    let split_iter = contents.split("\n");

    let pairs_vector: Vec<Vec<&str>> = split_iter
        .map(|entry| {
            let row: Vec<&str> = entry.split_terminator(" ").collect();
            row
        })
        .collect();

    let incorrect_score = calculate_incorrect_score(&pairs_vector);

    println!("{incorrect_score}");

    let correct_score = calculate_correct_score(pairs_vector);

    println!("{correct_score}");
}

// TODO implement enum for rock/paper/scissor

fn calculate_incorrect_score(pairs: &Vec<Vec<&str>>) -> u32 {
    let mut score = 0;

    for pair in pairs {
        if pair.len() != 2 {
            continue;
        }

        let opponent_choice = pair[0];
        let my_choice = pair[1];

        match my_choice {
            "X" => score += 1,
            "Y" => score += 2,
            "Z" => score += 3,
            _ => println!("My incorrect choice"),
        }

        if opponent_choice == "A" {
            match my_choice {
                "Z" => score += 0,
                "X" => score += 3,
                "Y" => score += 6,
                _ => println!("Couldn't compare"),
            }
        } else if opponent_choice == "B" {
            match my_choice {
                "X" => score += 0,
                "Y" => score += 3,
                "Z" => score += 6,
                _ => println!("Couldn't compare"),
            }
        } else if opponent_choice == "C" {
            match my_choice {
                "Y" => score += 0,
                "Z" => score += 3,
                "X" => score += 6,
                _ => println!("Couldn't compare"),
            }
        } else {
            println!("Incorrect opponent pattern");
        }
    }

    score
}

fn calculate_correct_score(pairs: Vec<Vec<&str>>) -> u32 {
    let mut score = 0;

    for pair in pairs {
        if pair.len() != 2 {
            continue;
        }

        let opponent_choice = pair[0];
        let round_result = pair[1];
        let mut my_choice = "";

        // Determines my choice given desired round_result
        match opponent_choice {
            "A" => match round_result {
                "X" => my_choice = "C",
                "Y" => my_choice = opponent_choice,
                "Z" => my_choice = "B",
                _ => println!("Couldn't figure my choice"),
            },
            "B" => match round_result {
                "X" => my_choice = "A",
                "Y" => my_choice = opponent_choice,
                "Z" => my_choice = "C",
                _ => println!("Couldn't figure my choice"),
            },
            "C" => match round_result {
                "X" => my_choice = "B",
                "Y" => my_choice = opponent_choice,
                "Z" => my_choice = "A",
                _ => println!("Couldn't figure my choice"),
            },
            _ => println!("Couldn't figure opponent choice"),
        };

        // Add score based on my choice
        match my_choice {
            "A" => score += 1,
            "B" => score += 2,
            "C" => score += 3,
            _ => println!("My incorrect choice"),
        }

        // Add score based on round result
        match round_result {
            "X" => score += 0,
            "Y" => score += 3,
            "Z" => score += 6,
            _ => println!("Couldn't figure round result"),
        }
    }

    score
}
