use std::collections::HashSet;

pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut priority_sum = 0;

    for line in lines {
        let rucksack_contents = line.trim();
        // Assuming 1 char = 1 byte since this is all ASCII.
        let number_of_items = rucksack_contents.len();

        if number_of_items == 0 {
            continue;
        }

        let halfway = number_of_items / 2;

        let first_compartment = get_hash_set_of_items(&rucksack_contents[..halfway]);
        let second_compartment = get_hash_set_of_items(&rucksack_contents[halfway..]);

        let intersection: Vec<&char> = first_compartment
            .intersection(&second_compartment)
            .collect();

        if intersection.len() != 1 {
            panic!("Intersection wrong size.");
        }

        let item_in_both_compartments = intersection[0];

        priority_sum += get_priority(*item_in_both_compartments);
    }

    priority_sum
}

pub fn solve_part_2(lines: &Vec<&str>) -> u32 {
    let mut priority_sum = 0;

    for group in lines.chunks_exact(3) {
        let first_rucksack = get_hash_set_of_items(group[0]);
        let second_rucksack = get_hash_set_of_items(group[1]);
        let third_rucksack = get_hash_set_of_items(group[2]);

        let intersection: Vec<&char> = first_rucksack
            .iter()
            .filter(|k| second_rucksack.contains(k))
            .filter(|k| third_rucksack.contains(k))
            .collect();

        if intersection.len() != 1 {
            panic!("Intersection wrong size.");
        }

        let common_item = intersection[0];

        priority_sum += get_priority(*common_item);
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

pub fn get_hash_set_of_items(items: &str) -> HashSet<char> {
    let mut hash_set = HashSet::new();
    for item in items.chars() {
        hash_set.insert(item);
    }
    return hash_set;
}
