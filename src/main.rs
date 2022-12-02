use std::fs;

mod day_1;
mod day_2;

fn main() {
    println!("===============================");
    println!("ANSWERS");
    println!("===============================");

    let day_1_input = fs::read_to_string("inputs/day_1.txt").expect("Error reading file.");
    let day_1_input = day_1_input.split('\n').collect();

    println!("Day 1, Part 1: {}", day_1::solve_part_1(&day_1_input));
    println!("Day 1, Part 2: {}", day_1::solve_part_2(&day_1_input));
    println!("-------------------------------");

    let day_2_input = fs::read_to_string("inputs/day_2.txt").expect("Error reading file.");
    let day_2_input = day_2_input.split('\n').collect();

    println!("Day 2, Part 1: {}", day_2::solve_part_1(&day_2_input));
    println!("Day 2, Part 2: {}", day_2::solve_part_2(&day_2_input));

    println!("===============================");
}
