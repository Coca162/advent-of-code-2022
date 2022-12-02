use phf::phf_map;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let semi_parsed = INPUT.split('\n').map(|game| 
        {
            let mut iter = game.chars();
            
            let opponent = iter.next().and_then(|c| OPPONENT_CHARS.get(&c)).unwrap();

            iter.next().unwrap();

            (opponent, iter.next().unwrap())
        }
    );

    // Part 1

    let part_1_parsed = semi_parsed.clone()
        .map(|(parsed, parsing)| (parsed, RESPONSE_CHOICE_CHARS.get(&parsing).unwrap().clone()));

    let score: u32 = part_1_parsed.map(|(opp, resp)| GameChoice::play(opp, resp)).sum();

    println!("{score}");

    // Part 2 

    let part_2_parsed = semi_parsed.clone()
        .map(|(parsed, parsing)| (parsed, END_STATE_CHARS.get(&parsing).unwrap().clone()));

    let actual_score: u32 = part_2_parsed.map(|(opp, state)| GameChoice::play_with_end(opp, state)).sum();

    println!("{actual_score}");
}

#[derive(Eq, PartialEq, Clone)]
enum GameChoice {
    Rock = 1,
    Paper = 2,
    Scissor = 3
}

#[derive(Clone)]
enum EndGameState {
    Win = 6,
    Draw = 3,
    Lose = 0
}

static OPPONENT_CHARS: phf::Map<char, GameChoice> = phf_map! {
    'A' => GameChoice::Rock,
    'B' => GameChoice::Paper,
    'C' => GameChoice::Scissor,
};

static RESPONSE_CHOICE_CHARS: phf::Map<char, GameChoice> = phf_map! {
    'X' => GameChoice::Rock,
    'Y' => GameChoice::Paper,
    'Z' => GameChoice::Scissor,
};

static END_STATE_CHARS: phf::Map<char, EndGameState> = phf_map! {
    'X' => EndGameState::Lose,
    'Y' => EndGameState::Draw,
    'Z' => EndGameState::Win,
};

impl GameChoice {
    pub fn play(opponent: &GameChoice, response: GameChoice) -> u32 {
        let round_points = {
            if &response == opponent {
                3
            }
            else if &response.winning_condition() == opponent {
                6
            }
            else {
                0
            }
        };

        round_points + response as u32
    }

    pub fn play_with_end(opponent: &GameChoice, end_game: EndGameState) -> u32 {
        let response = match end_game {
            EndGameState::Win => opponent.losing_condition(),
            EndGameState::Draw => opponent.clone(),
            EndGameState::Lose => opponent.winning_condition(),
        };

        response as u32 + end_game as u32
    }

    pub fn winning_condition(&self) -> Self {
        match self {
            Self::Paper => Self::Rock,
            Self::Scissor => Self::Paper,
            Self::Rock => Self::Scissor
        }
    }

    pub fn losing_condition(&self) -> Self {
        match self {
            Self::Paper => Self::Scissor,
            Self::Scissor => Self::Rock,
            Self::Rock => Self::Paper
        }
    }
}