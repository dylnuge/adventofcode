// Advent of Code Day 2, Part 2

use std::fs::File;
use std::io::{BufRead, BufReader};

static ROCK_OPP: &str = "A";
static PAPER_OPP: &str = "B";
static SCISSORS_OPP: &str = "C";
static LOSE_ME: &str = "X";
static DRAW_ME: &str = "Y";
static WIN_ME: &str = "Z";

pub enum Play {
    ROCK,
    PAPER,
    SCISSORS,
}

pub enum Outcome {
    WIN,
    LOSE,
    DRAW,
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

        let me: Outcome = if me_str == LOSE_ME {
            Outcome::LOSE
        } else if me_str == DRAW_ME {
            Outcome::DRAW
        } else if me_str == WIN_ME {
            Outcome::WIN
        } else {
            panic!("Bad me value: {}", me_str);
        };

        // Calculate win (6 points), tie (3 points), or loss (0 points)
        score += match &me {
            Outcome::LOSE => 0,
            Outcome::DRAW => 3,
            Outcome::WIN => 6,
        };

        // Calculate points for what we played: 1 rock, 2 paper, 3 scissors
        score += match (&opp, &me) {
            // Rocks
            (Play::ROCK, Outcome::DRAW) => 1,
            (Play::PAPER, Outcome::LOSE) => 1,
            (Play::SCISSORS, Outcome::WIN) => 1,
            // Papers
            (Play::ROCK, Outcome::WIN) => 2,
            (Play::PAPER, Outcome::DRAW) => 2,
            (Play::SCISSORS, Outcome::LOSE) => 2,
            // Scissors
            _ => 3,
        };
    }

    println!("Total score is {}", score);
}
