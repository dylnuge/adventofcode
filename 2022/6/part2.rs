// Advent of Code 2022 Day 6 Part 2
// Mostly identical to part 1

use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    // We know all the bytes are ASCII characters, but it doesn't really matter
    let data: Vec<u8> = fs::read("input.txt").unwrap();

    // Index 13 (the 14th byte in the stream) is the first place that we can
    // have a valid entry point, so we can just start there
    for byte_index in 13..data.len() {
        // Grab a slice of four bytes, ending at the current index (inclusive)
        let bytes = &data[byte_index - 13..=byte_index];
        // Convert it into a set. Worked out real well for part 2, since for
        // comparing a lot of bytes, this becomes pretty performant!
        let set: HashSet<u8> = HashSet::from_iter(bytes.iter().cloned());
        if set.len() == 14 {
            println!(
                "Sequence found!\n\tSequence is: {:?}\n\tEnds at byte: {}",
                bytes, byte_index
            );
            break;
        }
    }
}
