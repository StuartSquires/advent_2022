use std::fs;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

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
    println!("-------------------------------");

    let day_3_input = fs::read_to_string("inputs/day_3.txt").expect("Error reading file.");
    let day_3_input = day_3_input.split('\n').collect();

    println!("Day 3, Part 1: {}", day_3::solve_part_1(&day_3_input));
    println!("Day 3, Part 2: {}", day_3::solve_part_2(&day_3_input));
    println!("-------------------------------");

    let day_4_input = fs::read_to_string("inputs/day_4.txt").expect("Error reading file.");
    let day_4_input = day_4_input.split('\n').collect();

    println!("Day 4, Part 1: {}", day_4::solve_part_1(&day_4_input));
    println!("Day 4, Part 2: {}", day_4::solve_part_2(&day_4_input));
    println!("-------------------------------");

    let day_5_input = fs::read_to_string("inputs/day_5.txt").expect("Error reading file.");
    let day_5_input = day_5_input.split('\n').collect();

    println!("Day 5, Part 1: {}", day_5::solve_part_1(&day_5_input));
    println!("Day 5, Part 2: {}", day_5::solve_part_2(&day_5_input));

    println!("===============================");
}
