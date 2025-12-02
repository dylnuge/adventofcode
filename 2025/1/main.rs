use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut index: i64 = 50;
    let mut zero_count: u64 = 0;

    for line_result in reader.lines() {
        let line = line_result.unwrap();
        let (dir_str, count_str) = line.split_at(1);
        let direction = match dir_str {
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let count = count_str.parse::<i64>().unwrap();
        let init_zero = index == 0;

        match direction {
            Direction::Left => {
                index -= count;
            }
            Direction::Right => {
                index += count;
            }
        };

        // TODO(dylan): I wrote this all when I was tired coming in from a
        // flight. Might be good to take another pass; this ends up having more
        // complex edge cases than I'd like.

        // Annoying edge case: don't count starting on zero from this rotation
        // as passing it
        if index < 0 && init_zero {
            zero_count -= 1;
        }
        // Wind back around if we're out of the 0-99 range to the left, by
        // adding 100 until we're back in the range (and count each as a
        // rotation).
        while index < 0 {
            index += 100;
            zero_count += 1
        }
        // If we end on zero, count it here. This is also an annoying edge case:
        // it'd look nice to count this at the end, but we'd wind up counting an
        // extra time when coming back from the right because the first "0"
        // position we pass going to the right is actually 100.
        if index == 0 {
            zero_count += 1
        }
        // This is semantically equivalent to the left-side while loop, but we
        // can do it by just taking separately the quotient and remainder.
        if index >= 100 {
            zero_count += (index / 100) as u64;
            index = index % 100;
        }
    }

    println!("Zero count: {}", zero_count)
}
