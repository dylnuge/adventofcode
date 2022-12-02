// Advent of Code Day 2

use std::fs::File;
use std::io::{BufRead, BufReader};

static ROCK_OPP: &str = "A";
static PAPER_OPP: &str = "B";
static SCISSORS_OPP: &str = "C";
static ROCK_ME: &str = "X";
static PAPER_ME: &str = "Y";
static SCISSORS_ME: &str = "Z";

pub enum Play {
    ROCK,
    PAPER,
    SCISSORS,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut score = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();
        let plays: Vec<&str> = line_str.split_whitespace().collect();
        if plays.len() != 2 {
            continue;
        };
        let (opp_str, me_str) = (plays[0], plays[1]);

        let opp: Play = if opp_str == ROCK_OPP {
            Play::ROCK
        } else if opp_str == PAPER_OPP {
            Play::PAPER
        } else if opp_str == SCISSORS_OPP {
            Play::SCISSORS
        } else {
            panic!("Bad opponent value: {}", opp_str);
        };

        let me: Play = if me_str == ROCK_ME {
            Play::ROCK
        } else if me_str == PAPER_ME {
            Play::PAPER
        } else if me_str == SCISSORS_ME {
            Play::SCISSORS
        } else {
            panic!("Bad me value: {}", me_str);
        };

        // Calculate win (6 points), tie (3 points), or loss (0 points)
        score += match (&opp, &me) {
            // Wins
            (Play::ROCK, Play::PAPER) => 6,
            (Play::PAPER, Play::SCISSORS) => 6,
            (Play::SCISSORS, Play::ROCK) => 6,
            // Ties
            (Play::ROCK, Play::ROCK) => 3,
            (Play::PAPER, Play::PAPER) => 3,
            (Play::SCISSORS, Play::SCISSORS) => 3,
            // Losses
            _ => 0,
        };

        // Calculate points for what we played: 1 rock, 2 paper, 3 scissors
        score += match &me {
            Play::ROCK => 1,
            Play::PAPER => 2,
            Play::SCISSORS => 3,
        };
    }

    println!("Total score is {}", score);
}
