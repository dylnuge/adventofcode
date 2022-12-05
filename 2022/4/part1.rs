// Advent of Code Day 4 Part 1

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut count: usize = 0;

    for line in reader.lines() {
        let line_str = line.unwrap();

        // Skip blank lines
        if line_str == "" {
            continue;
        }

        let (elf1_str, elf2_str) = line_str.split_at(line_str.find(',').unwrap());
        let (elf1_left, elf1_right) = elf1_str.split_at(elf1_str.find('-').unwrap());
        let (elf2_left, elf2_right) = elf2_str.split_at(elf2_str.find('-').unwrap());

        let e1l = elf1_left.parse::<u32>().unwrap();
        let e2l = elf2_left[1..].parse::<u32>().unwrap();
        let e1r = elf1_right[1..].parse::<u32>().unwrap();
        let e2r = elf2_right[1..].parse::<u32>().unwrap();

        if (e1l >= e2l && e1r <= e2r) || (e2l >= e1l && e2r <= e1r) {
            count += 1;
        }
    }

    println!("{} overlapping pairs discovered", count);
}
