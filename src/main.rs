use std::fs;

mod day_1;

fn main() {
    let day_1_input = fs::read_to_string("inputs/day_1.txt").expect("Error reading file.");
    let day_1_input = day_1_input.split('\n').collect();

    println!("===============================");
    println!("Answers:");
    println!("===============================");
    println!("Day 1, Part 1: {}", day_1::solve_part_1(&day_1_input));
    println!("Day 1, Part 2: {}", day_1::solve_part_2(&day_1_input));
    println!("===============================");
}
