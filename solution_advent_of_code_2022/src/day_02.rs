// A rock => 1
// B paper => 2
// C scissors => 3
//
// X Rock => lose
// Y paper => draw
// Z scissors => win
//

pub fn calc_score_of_strat(input: &str) -> u32 {
    let parsed = parsed_input(input);
    let total_score = parsed
        .iter()
        .fold(0u32, |total, round| total + calc_one_round(&round));
    total_score
}

pub fn calc_score_outcome_strat(input: &str) -> u32 {
    let parsed = parsed_input(input);
    let total_score = parsed.iter().fold(0u32, |total, round| {
        total + calc_round_supposed_outcome(&round)
    });
    total_score
}

fn calc_round_supposed_outcome(round: &Round) -> u32 {
    let opponent = PlayerMoves::from_letter(&round.0);
    let counter = PlayerMoves::from_letter(&round.1);

    let supposed_outcome = counter.get_supposed_outcome();

    let supposed_move = match supposed_outcome {
        OutcomeScores::Win => find_winning_move(&opponent),
        OutcomeScores::Draw => opponent,
        OutcomeScores::Lose => find_losing_move(&opponent),
    };

    supposed_outcome.get_score() + supposed_move.get_score()
}

fn calc_one_round(round: &Round) -> u32 {
    let oppenent = PlayerMoves::from_letter(&round.0);
    let counter = PlayerMoves::from_letter(&round.1);
    let move_score = counter.get_score();

    if oppenent == counter {
        return OutcomeScores::Draw.get_score() + move_score;
    }

    let needed_counter = find_winning_move(&oppenent);

    if needed_counter == counter {
        OutcomeScores::Win.get_score() + move_score
    } else {
        OutcomeScores::Lose.get_score() + move_score
    }
}

fn find_losing_move(opponent: &PlayerMoves) -> PlayerMoves {
    match opponent {
        PlayerMoves::Paper => PlayerMoves::Rock,
        PlayerMoves::Rock => PlayerMoves::Scissors,
        PlayerMoves::Scissors => PlayerMoves::Paper,
    }
}

fn find_winning_move(opponent: &PlayerMoves) -> PlayerMoves {
    match opponent {
        PlayerMoves::Rock => PlayerMoves::Paper,
        PlayerMoves::Paper => PlayerMoves::Scissors,
        PlayerMoves::Scissors => PlayerMoves::Rock,
    }
}

const ROCK_SCORE: u32 = 1u32;
const PAPER_SCORE: u32 = 2u32;
const SCISSORS_SCORE: u32 = 3u32;
#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
enum PlayerMoves {
    Rock,
    Paper,
    Scissors,
}

const WIN_SCORE: u32 = 6u32;
const DRAW_SCORE: u32 = 3u32;
const LOSE_SCORE: u32 = 0u32;
enum OutcomeScores {
    Win,
    Draw,
    Lose,
}

impl OutcomeScores {
    fn get_score(&self) -> u32 {
        match self {
            Self::Win => WIN_SCORE,
            Self::Draw => DRAW_SCORE,
            Self::Lose => LOSE_SCORE,
        }
    }
}
impl PlayerMoves {
    fn from_letter(letter: &str) -> Self {
        match letter {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => panic!("No player move exits for letter {}", letter),
        }
    }

    fn get_score(&self) -> u32 {
        match self {
            Self::Rock => ROCK_SCORE,
            Self::Paper => PAPER_SCORE,
            Self::Scissors => SCISSORS_SCORE,
        }
    }

    fn get_supposed_outcome(&self) -> OutcomeScores {
        // X Rock => lose
        // Y paper => draw
        // Z scissors => win
        match self {
            Self::Rock => OutcomeScores::Lose,
            Self::Paper => OutcomeScores::Draw,
            Self::Scissors => OutcomeScores::Win,
        }
    }
}

#[derive(Debug)]
struct Round(String, String);
type ParsedInput = Vec<Round>;
fn parsed_input(to_parse: &str) -> ParsedInput {
    let mut parsed: ParsedInput = Vec::new();
    for line in to_parse.lines() {
        let mut left_right = line.split(' ');
        let left = left_right.next();
        let right = left_right.next();
        parsed.push(Round(
            left.expect("Did not find oppenents move as single letter")
                .to_owned(),
            right
                .expect("Did not find move to make as single letter")
                .to_owned(),
        ));
    }

    parsed
}
