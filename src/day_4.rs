pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut fully_contained_count = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let pair = parse_pair(line);

        if pair.first.contains(&pair.second) || pair.second.contains(&pair.first) {
            fully_contained_count += 1;
        }
    }

    fully_contained_count
}

pub fn solve_part_2(lines: &Vec<&str>) -> u32 {
    let mut overlap_count = 0;

    for line in lines {
        if line.is_empty() {
            continue;
        }

        let pair = parse_pair(line);

        if pair.first.overlaps(&pair.second) {
            overlap_count += 1;
        }
    }

    overlap_count
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

    fn overlaps(&self, other: &SectionAssignment) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

#[cfg(test)]
mod tests {
    use crate::day_4::SectionAssignment;

    #[test]
    fn overlaps_with_self() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 3, end: 5 };

        assert!(section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn overlaps_when_contained_by_other() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 2, end: 5 };

        assert!(section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn overlaps_when_containing_other() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 4 };
        let section_assignment_2 = SectionAssignment { start: 3, end: 5 };

        assert!(section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn overlaps_with_other_on_left() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 2, end: 3 };

        assert!(section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn overlaps_with_other_on_right() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 5, end: 6 };

        assert!(section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn does_not_overlap_with_other_on_left() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 1, end: 2 };

        assert!(!section_assignment_1.overlaps(&section_assignment_2));
    }

    #[test]
    fn does_not_overlap_with_other_on_right() {
        let section_assignment_1 = SectionAssignment { start: 3, end: 5 };
        let section_assignment_2 = SectionAssignment { start: 6, end: 7 };

        assert!(!section_assignment_1.overlaps(&section_assignment_2));
    }
}
