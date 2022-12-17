// Advent of Code 2022 Day 6 Part 1

use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    // We know all the bytes are ASCII characters, but it doesn't really matter
    let data: Vec<u8> = fs::read("input.txt").unwrap();

    // Index 3 (the 4th byte in the stream) is the first place that we can
    // have a valid entry point, so we can just start there
    for byte_index in 3..data.len() {
        // Grab a slice of four bytes, ending at the current index (inclusive)
        let bytes = &data[byte_index - 3..=byte_index];
        // Convert it into a set. There's some other ways to check uniqueness
        // here, and at four bytes this probably is less performant than just
        // doing six comparisons, but I like it, it's an Advent calender, I get
        // a little HashSet, as a treat.
        let set: HashSet<u8> = HashSet::from_iter(bytes.iter().cloned());
        if set.len() == 4 {
            println!(
                "Sequence found!\n\tSequence is: {:?}\n\tEnds at byte: {}",
                bytes, byte_index
            );
            break;
        }
    }
}
