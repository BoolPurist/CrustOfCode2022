// A rock => 1
// B paper => 2
// C scissors => 3
//
// X Rock
// Y paper
// Z scissors
//

use std::collections::HashMap;

pub fn calc_score_of_strat(input: &str) -> u32 {
    let parsed = parsed_input(input);
    let table = get_beat_table();
    let total_score = parsed
        .iter()
        .fold(0u32, |total, round| total + calc_one_round(&round, &table));
    total_score
}

fn calc_one_round(round: &Round, lookup_table: &BeatTable) -> u32 {
    let oppenent = PlayerMoves::from_letter(&round.0);
    let counter = PlayerMoves::from_letter(&round.1);
    let move_score = counter.get_score();
    if oppenent == counter {
        return OutcomeScores::Draw.get_score() + move_score;
    }

    if let Some(needed_counter) = lookup_table.get(&oppenent) {
        return if needed_counter == &counter {
            OutcomeScores::Win.get_score() + move_score
        } else {
            OutcomeScores::Lose.get_score() + move_score
        };
    };

    panic!("No needed counter found for {:?}", oppenent);
}

const ROCK_SCORE: u32 = 1u32;
const PAPER_SCORE: u32 = 2u32;
const SCISSORS_SCORE: u32 = 3u32;
#[derive(Eq, Hash, PartialEq, Debug)]
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
}
type BeatTable = HashMap<PlayerMoves, PlayerMoves>;
fn get_beat_table() -> BeatTable {
    let mut table: BeatTable = HashMap::new();
    table.insert(PlayerMoves::Rock, PlayerMoves::Scissors);
    table.insert(PlayerMoves::Paper, PlayerMoves::Rock);
    table.insert(PlayerMoves::Scissors, PlayerMoves::Paper);
    table
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
