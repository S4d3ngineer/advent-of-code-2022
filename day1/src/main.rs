use std::fs;

fn main() {
    let file_path = "src/calory-data.txt";

    println!("{}", file_path);

    let contents = fs::read_to_string(file_path).expect("Couldn't read the file");

    let split = contents
        .split_terminator("\n\n")
        .map(|entry| {
            entry
                .split_terminator("\n")
                .map(|calory| calory.parse::<u32>().expect("Couldn't parse {calory}"))
                .sum::<u32>()
        })
        .max()
        .expect("Catfish");

    println!("{split}");
}
