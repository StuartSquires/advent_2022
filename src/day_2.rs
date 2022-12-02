pub fn solve_part_1(lines: &Vec<&str>) -> u32 {
    let mut total_score = 0;

    for line in lines {
        let moves: Vec<&str> = line.trim().split(' ').collect();

        // Handle an empty line at end of file.
        if moves.len() < 2 {
            continue;
        }

        let theirs = moves[0];
        let ours = moves[1];

        let theirs = match theirs {
            "A" => Move::ROCK,
            "B" => Move::PAPER,
            "C" => Move::SCISSORS,
            _ => panic!("Invalid move!"),
        };

        let ours = match ours {
            "X" => Move::ROCK,
            "Y" => Move::PAPER,
            "Z" => Move::SCISSORS,
            _ => panic!("Invalid move!"),
        };

        let outcome = get_outcome(&ours, &theirs);
        total_score += get_shape_score(&ours) + get_outcome_score(&outcome);
    }

    total_score
}

pub fn solve_part_2(lines: &Vec<&str>) -> u32 {
    let mut total_score = 0;

    for line in lines {
        let line: Vec<&str> = line.trim().split(' ').collect();

        // Handle an empty line at end of file.
        if line.len() < 2 {
            continue;
        }

        let theirs = line[0];
        let target = line[1];

        let theirs = match theirs {
            "A" => Move::ROCK,
            "B" => Move::PAPER,
            "C" => Move::SCISSORS,
            _ => panic!("Invalid move!"),
        };

        let target = match target {
            "X" => Outcome::LOSE,
            "Y" => Outcome::DRAW,
            "Z" => Outcome::WIN,
            _ => panic!("Invalid target!"),
        };

        let ours = get_move(&theirs, &target);

        total_score += get_shape_score(&ours) + get_outcome_score(&target);
    }

    total_score
}

fn get_shape_score(ours: &Move) -> u32 {
    match ours {
        Move::ROCK => 1,
        Move::PAPER => 2,
        Move::SCISSORS => 3,
    }
}

fn get_outcome(ours: &Move, theirs: &Move) -> Outcome {
    match ours {
        Move::ROCK => match theirs {
            Move::ROCK => Outcome::DRAW,
            Move::PAPER => Outcome::LOSE,
            Move::SCISSORS => Outcome::WIN,
        },
        Move::PAPER => match theirs {
            Move::ROCK => Outcome::WIN,
            Move::PAPER => Outcome::DRAW,
            Move::SCISSORS => Outcome::LOSE,
        },
        Move::SCISSORS => match theirs {
            Move::ROCK => Outcome::LOSE,
            Move::PAPER => Outcome::WIN,
            Move::SCISSORS => Outcome::DRAW,
        },
    }
}

fn get_outcome_score(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::WIN => 6,
        Outcome::DRAW => 3,
        Outcome::LOSE => 0,
    }
}

fn get_move(theirs: &Move, target: &Outcome) -> Move {
    match theirs {
        Move::ROCK => match target {
            Outcome::WIN => Move::PAPER,
            Outcome::DRAW => Move::ROCK,
            Outcome::LOSE => Move::SCISSORS,
        },
        Move::PAPER => match target {
            Outcome::WIN => Move::SCISSORS,
            Outcome::DRAW => Move::PAPER,
            Outcome::LOSE => Move::ROCK,
        },
        Move::SCISSORS => match target {
            Outcome::WIN => Move::ROCK,
            Outcome::DRAW => Move::SCISSORS,
            Outcome::LOSE => Move::PAPER,
        },
    }
}

enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

enum Outcome {
    WIN,
    DRAW,
    LOSE,
}
