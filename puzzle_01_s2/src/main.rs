use std::fs; // To read files
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    
    println!("In file {file_path} we have: ");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|num|num.parse().expect("It should be a number"))
            .collect();

        if numbers.len() >= 2 { // Verifies line is not empty
            list1.push(numbers[0]);
            list2.push(numbers[1]);
        }
    }

    let mut right_counts = HashMap::new(); // Defines data structure hash map

    for number in &list2 {
        *right_counts.entry(*number).or_insert(0) += 1;
    }

    let mut similarity_score = 0;
    for number in &list1 {
        let count = right_counts.get(number).unwrap_or(&0);
        similarity_score += number * count;
    }    

    println!("La puntuación de similitud es {similarity_score}");
}
