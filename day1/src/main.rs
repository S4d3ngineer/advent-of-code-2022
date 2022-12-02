use std::fs;

fn main() {
    let file_path = "src/calory-data.txt";

    let contents = fs::read_to_string(file_path).expect("Couldn't read the file");

    let max_calories = contents
        .split_terminator("\n\n")
        .map(|entry| {
            entry
                .split_terminator("\n")
                .map(|calory| calory.parse::<u32>().expect("Couldn't parse {calory}"))
                .sum::<u32>()
        })
        .max()
        .expect("Couldn't calculate max value");

    println!("{max_calories}");
}
