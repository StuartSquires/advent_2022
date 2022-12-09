use std::collections::HashSet;

pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut priority_sum = 0;

    for line in lines {
        let rucksack_contents: Vec<char> = line.trim().chars().collect();
        // Assuming 1 char = 1 byte since this is all ASCII.
        let number_of_items = rucksack_contents.len();

        if number_of_items == 0 {
            continue;
        }

        let halfway = number_of_items / 2;

        let mut first_compartment = HashSet::new();
        for ch in rucksack_contents[..halfway].iter() {
            first_compartment.insert(ch);
        }

        let mut second_compartment = HashSet::new();
        for ch in rucksack_contents[halfway..].iter() {
            second_compartment.insert(ch);
        }

        let intersection: Vec<&&char> = first_compartment
            .intersection(&second_compartment)
            .collect();

        if intersection.len() != 1 {
            panic!("Intersection wrong size.");
        }

        let item_in_both_compartments = intersection[0];

        priority_sum += get_priority(**item_in_both_compartments);
    }

    priority_sum
}

pub fn get_priority(ch: char) -> u32 {
    if !ch.is_ascii() {
        panic!("Invalid item.");
    }

    let ascii_value = ch as u32;

    return if ch.is_lowercase() {
        ascii_value - 96
    } else {
        ascii_value - 65 + 27
    };
}
