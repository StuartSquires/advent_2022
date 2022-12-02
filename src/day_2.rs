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

        total_score += get_shape_score(&ours) + get_outcome_score(&ours, &theirs);
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

fn get_outcome_score(ours: &Move, theirs: &Move) -> u32 {
    match ours {
        Move::ROCK => match theirs {
            Move::ROCK => DRAW_SCORE,
            Move::PAPER => LOSE_SCORE,
            Move::SCISSORS => WIN_SCORE,
        },
        Move::PAPER => match theirs {
            Move::ROCK => WIN_SCORE,
            Move::PAPER => DRAW_SCORE,
            Move::SCISSORS => LOSE_SCORE,
        },
        Move::SCISSORS => match theirs {
            Move::ROCK => LOSE_SCORE,
            Move::PAPER => WIN_SCORE,
            Move::SCISSORS => DRAW_SCORE,
        },
    }
}

enum Move {
    ROCK,
    PAPER,
    SCISSORS,
}

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;
