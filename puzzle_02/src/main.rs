use std::fs;

fn main() {
    let input_file = "input.txt";

    let contents = fs::read_to_string(input_file).expect("File could not be opened");

    let mut safe_count = 0;
    let mut increasing = true;

    for line in contents.lines() {
        let words: Vec<i32> = line.split_whitespace().map(|num|num.parse().expect("It should be a number")).collect();
        for i in 1..words.len() {
            if i == i {
                if words[i] > words[i-1] {
                    increasing = true;
                } else {
                    increasing = false;
                }
            } else {
                if increasing {

                }


            }


            let difference = (words[i] - words[i-1]).abs();
            if difference >= 1 && difference <= 3 {
                safe_count += 1; 
            }
            println!("{}", difference);
        }
        println!();
    }

    println!("Safes counts: {}", safe_count);
}
