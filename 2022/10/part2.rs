// Advent of Code 2022 Day 10 Part 2

use std::fs;
use std::io;
use std::io::Write;

fn draw_cycle(cycle: isize, reg_x: isize) {
    let x_pos = (cycle - 1) % 40;
    // Determine which character to print
    let pixel = if x_pos == reg_x - 1 || x_pos == reg_x || x_pos == reg_x + 1 {
        "#"
    } else {
        "."
    };
    print!("{}", pixel);

    if cycle % 40 == 0 {
        print!("\n");
        io::stdout().flush();
    }
}

fn main() {
    let instructions = fs::read_to_string("input.txt").unwrap();

    // Processor values: cycle count and X register value.
    let mut reg_x = 1isize;
    let mut cycle = 0isize;

    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace();

        match parts.next().unwrap() {
            "noop" => {
                cycle += 1;
                draw_cycle(cycle, reg_x);
            }
            "addx" => {
                cycle += 1;
                draw_cycle(cycle, reg_x);
                cycle += 1;
                draw_cycle(cycle, reg_x);

                // Addition occurs *after* we've processed both cycles
                let val = parts.next().unwrap().parse::<isize>().unwrap();
                reg_x += val;
            }
            ins => panic!("unknown instruction: {}", ins),
        }
    }
}
