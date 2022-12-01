use std::io;

fn main() {
    let lines = io::stdin().lines();

    let mut maximum_elf_calories = 0;
    let mut current_elf_calories = 0;
    for line in lines {
        let line = line.unwrap();

        if line.trim() == "" {
            if current_elf_calories > maximum_elf_calories {
                maximum_elf_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue;
        }

        let calories: u32 = line.trim().parse().expect("Calories must be a number.");

        current_elf_calories += calories;

        println!("got a line: {}", line)
    }

    if current_elf_calories > maximum_elf_calories {
        maximum_elf_calories = current_elf_calories;
    }

    println!("Answer: {}", maximum_elf_calories);
}
