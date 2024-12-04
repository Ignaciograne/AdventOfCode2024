use std::fs; // To read files

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
    
    list1.sort();
    list2.sort();

    let total_distance: i32 = list1.iter()
        .zip(list2.iter())
        .map(|(&left, &right)| (left-right).abs() )
        .sum();

    println!("Total distance is {total_distance}");
}
