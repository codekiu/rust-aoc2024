use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").expect("Should read the file");

    let mut left_vector: Vec<i64> = Vec::new();
    let mut right_vector: Vec<i64> = Vec::new();

    let mut number_with_repetitions = HashMap::new();
    let mut total_part_two = 0;

    for line in input.split("\n") {
        if line.is_empty() {
            break;
        }

        let mut column = line.split("   ");

        let first_value = column.next();
        let second_value = column.next();

        let left_value = first_value.unwrap().parse::<i64>().unwrap();
        let right_value = second_value.unwrap().parse::<i64>().unwrap();

        left_vector.push(left_value);
        right_vector.push(right_value);

        let count = number_with_repetitions.entry(right_value).or_insert(0);
        *count += 1;

        println!("Line; {} - {}", first_value.unwrap().trim(), second_value.unwrap().trim());
    }



    left_vector.sort();
    right_vector.sort();

    let mut result = 0;
    let length = left_vector.len();

    for idx in 0..length {
        if left_vector[idx] > right_vector[idx] {
            result += left_vector[idx] - right_vector[idx];
        } else {
            result += right_vector[idx] - left_vector[idx];
        }

        total_part_two += left_vector[idx] * number_with_repetitions.get(&left_vector[idx]).copied().unwrap_or(0);
    }

    println!("Result is = {}", result);
    println!("Result for part two is = {}", total_part_two);
}
