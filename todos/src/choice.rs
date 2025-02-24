#[derive(Debug,PartialEq)]
pub enum Choice {
    ROCK,
    PAPER,
    SCISSORS,
}

pub fn generate_choice() -> Choice {
    let rand_no = rand::random_range(0..3);

    match rand_no {
        0 => Choice::ROCK,
        1 => Choice::PAPER,
        _ => Choice::SCISSORS,
    }
}

pub fn generate_user_choice(input: &str) -> Option<Choice> {
    match input.trim().to_lowercase().as_str() {
        "rock" => Some(Choice::ROCK),
        "paper" => Some(Choice::PAPER),
        "scissors" => Some(Choice::SCISSORS),
        _ => None,
    }
}