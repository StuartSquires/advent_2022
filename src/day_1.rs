pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut maximum_elf_calories = 0;
    let mut current_elf_calories = 0;
    for line in lines {
        if line.trim() == "" {
            if current_elf_calories > maximum_elf_calories {
                maximum_elf_calories = current_elf_calories;
            }
            current_elf_calories = 0;
            continue;
        }

        let calories: u32 = line.trim().parse().expect("Calories must be a number.");

        current_elf_calories += calories;
    }

    if current_elf_calories > maximum_elf_calories {
        maximum_elf_calories = current_elf_calories;
    }

    maximum_elf_calories
}

pub fn solve_part_2(lines: &Vec<&str>) -> u32 {
    let mut top_three = TopThree {
        first: 0,
        second: 0,
        third: 0,
    };
    let mut current_elf_calories = 0;
    for line in lines {
        if line.trim() == "" {
            top_three = top_three.maybe_insert(current_elf_calories);
            current_elf_calories = 0;
            continue;
        }

        let calories: u32 = line.trim().parse().expect("Calories must be a number.");

        current_elf_calories += calories;
    }

    top_three = top_three.maybe_insert(current_elf_calories);

    top_three.first + top_three.second + top_three.third
}

struct TopThree {
    first: u32,
    second: u32,
    third: u32,
}

impl TopThree {
    fn maybe_insert(&self, new: u32) -> TopThree {
        if new > self.first {
            return TopThree {
                first: new,
                second: self.first,
                third: self.second,
            };
        } else if new > self.second {
            return TopThree {
                first: self.first,
                second: new,
                third: self.second,
            };
        } else if new > self.third {
            return TopThree {
                first: self.first,
                second: self.second,
                third: new,
            };
        }

        return TopThree {
            first: self.first,
            second: self.second,
            third: self.third,
        };
    }
}
