pub fn solve_part_1(lines: &Vec<&str>) -> String {
    let stack_lines = &lines[..8];

    let mut stacks: [Vec<char>; 9] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for line in stack_lines.iter().rev() {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..9 {
            let position = (i * 4) + 1;
            let char = chars[position];
            if char != ' ' {
                stacks[i].push(char);
            }
        }
    }

    let move_lines = &lines[10..];
    for line in move_lines.iter() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(' ').collect();

        let count: u32 = parts[1].parse().expect("Failed to parse count.");
        let from: usize = parts[3].parse().expect("Failed to parse from.");
        let to: usize = parts[5].parse().expect("Failed to parse to.");

        for _ in 0..count {
            let item = stacks[from - 1].pop().expect("Stack was empty.");
            stacks[to - 1].push(item);
        }
    }

    stacks
        .map(|stack| *stack.last().expect("Stack was empty."))
        .iter()
        .collect()
}
