// Advent of Code 2022 Day 10 Part 1

use std::fs;

fn main() {
    let instructions = fs::read_to_string("input.txt").unwrap();

    // Processor values: cycle count and X register value.
    let mut reg_x = 1isize;
    let mut cycle = 0isize;

    // Sum of signals
    let mut signal_sum = 0isize;

    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace();

        match parts.next().unwrap() {
            "noop" => {
                cycle += 1;
                if cycle == 20 || (20 - cycle) % 40 == 0 {
                    signal_sum += cycle * reg_x;
                }
            }
            "addx" => {
                cycle += 1;
                if cycle == 20 || (20 - cycle) % 40 == 0 {
                    signal_sum += cycle * reg_x;
                }
                cycle += 1;
                if cycle == 20 || (20 - cycle) % 40 == 0 {
                    signal_sum += cycle * reg_x;
                }
                let val = parts.next().unwrap().parse::<isize>().unwrap();
                reg_x += val;
            }
            ins => panic!("unknown instruction: {}", ins),
        }
    }

    println!("Signal strengths: {}", signal_sum);
}
