pub fn solve_part_1(lines: &Vec<&str>) -> String {
    let mut stacks = init_stacks(lines);

    let move_lines = &lines[10..];
    for line in move_lines.iter() {
        if line.is_empty() {
            continue;
        }

        let current_move = get_move(line);

        for _ in 0..current_move.count {
            let item = stacks[current_move.from - 1]
                .pop()
                .expect("Stack was empty.");
            stacks[current_move.to - 1].push(item);
        }
    }

    get_top_crates(stacks)
}

pub fn solve_part_2(lines: &Vec<&str>) -> String {
    let mut stacks = init_stacks(lines);

    let move_lines = &lines[10..];
    for line in move_lines.iter() {
        if line.is_empty() {
            continue;
        }

        let current_move = get_move(line);
        let mut tmp_stack = Vec::new();
        for _ in 0..current_move.count {
            let item = stacks[current_move.from - 1]
                .pop()
                .expect("Stack was empty.");
            tmp_stack.push(item);
        }

        for _ in 0..current_move.count {
            let item = tmp_stack.pop().expect("Stack was empty.");
            stacks[current_move.to - 1].push(item);
        }
    }

    get_top_crates(stacks)
}

fn init_stacks(lines: &Vec<&str>) -> [Vec<char>; 9] {
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

    stacks
}

fn get_move(line: &str) -> Move {
    let parts: Vec<&str> = line.split(' ').collect();

    Move {
        count: parts[1].parse().expect("Failed to parse count."),
        from: parts[3].parse().expect("Failed to parse from."),
        to: parts[5].parse().expect("Failed to parse to."),
    }
}

fn get_top_crates(stacks: [Vec<char>; 9]) -> String {
    stacks
        .map(|stack| *stack.last().expect("Stack was empty."))
        .iter()
        .collect()
}

struct Move {
    count: usize,
    from: usize,
    to: usize,
}
