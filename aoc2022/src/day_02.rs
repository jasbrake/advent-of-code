enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl Rps {
    fn parse(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("not a valid move"),
        }
    }

    fn value(&self) -> i32 {
        match *self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }

    fn play(moves: (&Self, &Self)) -> (i32, i32) {
        let their_move_value = moves.0.value();
        let our_move_value = moves.1.value();

        let score = match moves {
            (Rps::Rock, Rps::Rock) => (3, 3),     // Tie
            (Rps::Rock, Rps::Paper) => (0, 6),    // We win
            (Rps::Rock, Rps::Scissors) => (6, 0), // They win
                                                      
            (Rps::Paper, Rps::Rock) => (6, 0),     // They win
            (Rps::Paper, Rps::Paper) => (3, 3),    // Tie
            (Rps::Paper, Rps::Scissors) => (0, 6), // We win
                                                     
            (Rps::Scissors, Rps::Rock) => (0, 6),     // We win
            (Rps::Scissors, Rps::Paper) => (6, 0),    // They win
            (Rps::Scissors, Rps::Scissors) => (3, 3), // Tie
        };

        (score.0 + their_move_value, score.1 + our_move_value)
    }
}

#[derive(Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}


impl Outcome {
    fn parse(s: &str) -> Self {
        match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("not a valid move"),
        }
    }
    
    fn needed_move(&self, given_move: &Rps) -> Rps {
        match (*self, given_move) {
            (Outcome::Lose, Rps::Rock) => Rps::Scissors,
            (Outcome::Lose, Rps::Paper) => Rps::Rock,
            (Outcome::Lose, Rps::Scissors) => Rps::Paper,
            (Outcome::Draw, Rps::Rock) => Rps::Rock,
            (Outcome::Draw, Rps::Paper) => Rps::Paper,
            (Outcome::Draw, Rps::Scissors) => Rps::Scissors,
            (Outcome::Win, Rps::Rock) => Rps::Paper,
            (Outcome::Win, Rps::Paper) => Rps::Scissors,
            (Outcome::Win, Rps::Scissors) => Rps::Rock,
        }
    }
}

#[aoc_generator(day2, part1)]
fn input_generator(input: &str) -> Vec<(Rps, Rps)> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            (
                Rps::parse(iter.next().unwrap()),
                Rps::parse(iter.next().unwrap()),
            )
        })
        .collect()
}

#[aoc_generator(day2, part2)]
fn input_generator2(input: &str) -> Vec<(Rps, Outcome)> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split_whitespace();
            (
                Rps::parse(iter.next().unwrap()),
                Outcome::parse(iter.next().unwrap()),
            )
        })
        .collect()
}

#[aoc(day2, part1)]
fn p1(input: &[(Rps, Rps)]) -> i32 {
    input
        .iter()
        .fold(0, |score, (them, us)| score + Rps::play((them, us)).1)
}

#[aoc(day2, part2)]
fn p2(input: &[(Rps, Outcome)]) -> i32 {
    input
        .iter()
        .map(|(m, outcome)| (m, outcome.needed_move(m)))
        .fold(0, |score, (them, us)| score + Rps::play((them, &us)).1)
}
