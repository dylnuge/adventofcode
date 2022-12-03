// Advent of Code Day 3 Part 2

use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn prioritize_item(item: &char) -> u32 {
    if *item >= 'a' && *item <= 'z' {
        // a-z have priorities 1-26
        (*item as u8 - b'a' + 1) as u32
    } else if *item >= 'A' && *item <= 'Z' {
        // A-Z have priorities 27-52
        (*item as u8 - b'A' + 27) as u32
    } else {
        panic!("Invalid item in input: {}", item)
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut priority_sum: u32 = 0;
    loop {
        let mut line1 = String::new();
        let mut line2 = String::new();
        let mut line3 = String::new();
        let size1 = reader.read_line(&mut line1);
        // exit the loop when we hit EOF
        if let Ok(0) = size1 {
            break;
        }
        // Assume we cannot hit EOF or fail to read in the middle of the group
        reader
            .read_line(&mut line2)
            .expect("Error reading mid-group");
        reader
            .read_line(&mut line3)
            .expect("Error reading mid-group");

        let elf1_items: HashSet<char> = line1.trim().chars().collect();
        let elf2_items: HashSet<char> = line2.trim().chars().collect();
        let elf3_items: HashSet<char> = line3.trim().chars().collect();

        // TODO this is extremely ugly, and the weird usage of references here
        // is the result of wrestling with the borrow checker instead of
        // understanding why exactly it is that BitAnd expects a reference to
        // HashSets and such. The fact that this is so much messier than the
        // two way intersection code is a pain; want to come back and see how
        // this can be cleaned up
        let common_items = &(&elf1_items & &elf2_items) & &elf3_items;

        // Assert there's only one badge
        assert!(common_items.len() == 1);
        let badge = common_items.iter().collect::<Vec<&char>>()[0];

        priority_sum += prioritize_item(badge);
    }

    println!("Sum of badge priorities: {}", priority_sum);
}
