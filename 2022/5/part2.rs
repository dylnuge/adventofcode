// Advent of Code Day 5 Part 1

use std::fs::File;
use std::io::{BufRead, BufReader};

fn build_stacks(reader: &mut BufReader<File>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut stack_count: usize = 0;

    // This is an annoying format to work with, but we know that each
    // stack contains a single ASCII capital alphabetic character and that
    // the stacks are numbered sequentially starting with 1, so we can
    // read the "stack contents" lines until we arrive at any line with a 1
    // on it, then read that as our stack "positioning lines."
    let mut stack_contents_lines: Vec<String> = Vec::new();
    loop {
        let mut current_line = String::new();
        // Unwrap the length to panic if the read fails
        reader.read_line(&mut current_line).unwrap();

        if let Some(_) = current_line.find('1') {
            let stack_nums = current_line.trim().split(' ');
            for stack_num in stack_nums {
                // There will also be intermediate spaces in here, so ensure
                // we're looking at a number.
                if let Ok(num) = stack_num.parse::<usize>() {
                    stack_count += 1;
                    // Sanity check that nums are sequential
                    assert!(stack_count == num);
                }
            }
            break;
        } else {
            stack_contents_lines.push(current_line);
        }
    }

    // The file format is ambiguously if there's double digit stack numbers,
    // because the stacks themselves are a single character wide (plus the two
    // bracket characters).
    assert!(stack_count <= 9);
    for _ in 0..stack_count {
        stacks.push(Vec::new());
    }

    // Since we pushed the lines onto a vec, popping them will give them to
    // us in the reverse order (bottom first), which is what we want (so we
    // can push them onto another stack, it's stacks on stacks on stacks).
    while let Some(line) = stack_contents_lines.pop() {
        let line_bytes = line.as_bytes();
        for i in 0..stack_count {
            if line_bytes[i*4] == '[' as u8 {
                stacks[i].push(line_bytes[i*4 + 1] as char);
            }
        }
    }
    
    stacks
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut reader = BufReader::new(file);

    // Read the stacks
    let mut stacks = build_stacks(&mut reader);

    // Do the move operations
    for line_result in reader.lines() {
        let line = line_result.unwrap();

        // Skip empty lines
        if line.len() == 0 {
            continue;
        }

        // Line format should be "move %d from %d to %d"
        let mut parts = line.trim().split(' ');
        assert!(parts.next() == Some("move"));
        let count = parts.next().unwrap().parse::<usize>().unwrap();
        assert!(parts.next() == Some("from"));
        // Src and dst are 1-indexed in the input, but stacks is 0-indexed
        let src = parts.next().unwrap().parse::<usize>().unwrap() - 1;
        assert!(parts.next() == Some("to"));
        let dst = parts.next().unwrap().parse::<usize>().unwrap() - 1;

        // ONLY CHANGE FROM PART 1
        // Moves are now done together
        let mut items = Vec::new();
        for _ in 0..count {
            items.push(stacks[src].pop().unwrap())
        }
        while let Some(item) = items.pop() {
            stacks[dst].push(item);
        }
    } 

    println!("{:?}", stacks);
}