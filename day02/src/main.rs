use std::io::Lines;
use std::time::SystemTime;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}


struct Round {
    player: Move,
    opponent: Move,
}


enum Result {
    Win,
    Loss,
    Draw,
}

impl Result {
    fn from_string(c: &str) -> Option<Self> {
        match c {
            "X" => Some(Result::Loss),
            "Y" => Some(Result::Draw),
            "Z" => Some(Result::Win),
            _ => None,
        }
    }
}

impl Round {
    fn new(player: Move, opponent: Move) -> Self {
        Round { player, opponent }
    }

    fn from_player_outcome(opponent: Move, outcome: Result) -> Self {
        match outcome {
            Result::Win => Round::new(opponent.winning_opposite(), opponent),
            Result::Loss => Round::new(opponent.loosing_opposite(), opponent),
            Result::Draw => Round::new(opponent, opponent),
        }
    }

    fn winner(&self) -> Result {
        match (&self.player, &self.opponent) {
            (Move::Rock, Move::Scissors) => Result::Win,
            (Move::Paper, Move::Rock) => Result::Win,
            (Move::Scissors, Move::Paper) => Result::Win,
            (Move::Rock, Move::Paper) => Result::Loss,
            (Move::Paper, Move::Scissors) => Result::Loss,
            (Move::Scissors, Move::Rock) => Result::Loss,
            _ => Result::Draw,
        }
    }

    fn score(&self) -> i32 {
        let shape_score = match self.player {
            Move::Rock => { 1 }
            Move::Paper => { 2 }
            Move::Scissors => { 3 }
        };
        let result_score = match self.winner() {
            Result::Win => { 6 }
            Result::Draw => { 3 }
            Result::Loss => { 0 }
        };

        shape_score + result_score
    }
}

impl Move {
    fn from_string(c: &str) -> Option<Self> {
        match c {
            "A" => Some(Move::Rock),
            "B" => Some(Move::Paper),
            "C" => Some(Move::Scissors),
            "X" => Some(Move::Rock),
            "Y" => Some(Move::Paper),
            "Z" => Some(Move::Scissors),
            _ => None,
        }
    }

    fn winning_opposite(self) -> Self {
        match self {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        }
    }

    fn loosing_opposite(self) -> Self {
        match self {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        }
    }
}

fn main() {
    let begin = SystemTime::now();
    let score_part1 = INPUT.lines().map(|line| line.split_whitespace().map(|mov| Move::from_string(mov).unwrap()).collect::<Vec<_>>())
        .map(|moves| Round::new(moves[1], moves[0]))
        .map(|round| round.score())
        .sum::<i32>();
    println!("Score part1: {}", score_part1);
    let end = SystemTime::now();

    let score_part2 = INPUT.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let opponent = Move::from_string(iter.next().unwrap()).unwrap();
        let outcome = Result::from_string(iter.next().unwrap()).unwrap();
        Round::from_player_outcome(opponent, outcome)
    }).map(|round| round.score())
        .sum::<i32>();
    println!("Score part2: {}", score_part2);
    println!("Time: {:?}", end.duration_since(begin).unwrap());
}

