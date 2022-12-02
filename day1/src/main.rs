use std::fs;

fn main() {
    let file_path = "src/calory-data.txt";

    let contents = fs::read_to_string(file_path).expect("Couldn't read the file");

    let split_iter = contents.split("\n\n");

    let calories_iter = split_iter.map(|entry| {
        entry
            .split_terminator("\n")
            .map(|calory| calory.parse::<u32>().expect("Couldn't parse {calory}"))
            .sum::<u32>()
    });

    let max_calories = calories_iter
        .clone()
        .max()
        .expect("Couldn't calculate max value");

    println!("{:?}", max_calories);

    let mut calories_vector = calories_iter.collect::<Vec<u32>>();
    calories_vector.sort();
    calories_vector.reverse();

    let top_3_calories_total = &calories_vector[0..3].iter().sum::<u32>(); 

    println!("{:?}", top_3_calories_total); 
}
