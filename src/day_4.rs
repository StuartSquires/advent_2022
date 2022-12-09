pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut fully_contained_count = 0;

    for line in lines {
        if (line.is_empty()) {
            continue;
        }

        let pair = parse_pair(line);

        if pair.first.contains(&pair.second) || pair.second.contains(&pair.first) {
            fully_contained_count += 1;
        }
    }

    fully_contained_count
}

fn parse_pair(line: &str) -> Pair {
    let pair: Vec<&str> = line.trim().split(',').collect();
    Pair {
        first: parse_section_assignment(pair[0]),
        second: parse_section_assignment(pair[1]),
    }
}

fn parse_section_assignment(sections: &str) -> SectionAssignment {
    let sections: Vec<&str> = sections.split('-').collect();
    SectionAssignment {
        start: sections[0].parse().expect("Invalid number."),
        end: sections[1].parse().expect("Invalid number."),
    }
}

struct Pair {
    first: SectionAssignment,
    second: SectionAssignment,
}

struct SectionAssignment {
    start: u32,
    end: u32,
}

impl SectionAssignment {
    fn contains(&self, other: &SectionAssignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}
