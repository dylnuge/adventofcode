// Advent of Code Day 9

use std::collections::HashSet;
use std::fs;

// as an invarient, the tail can be assumed to never be more than two squares
// away from the head
fn move_tail(head_x: i32, head_y: i32, tail_x: i32, tail_y: i32) -> (i32, i32) {
    let x_dist = (tail_x - head_x).abs();
    let y_dist = (tail_y - head_y).abs();
    if x_dist <= 1 && y_dist <= 1 {
        // Tail is within 1 square of head (including on top of it), doesn't
        // move
        return (tail_x, tail_y);
    }

    // Move a square left or right towards the head
    let new_x = if tail_x < head_x {
        tail_x + 1
    } else if tail_x > head_x {
        tail_x - 1
    } else {
        tail_x
    };

    // Move a square up or down towards the head
    let new_y = if tail_y < head_y {
        tail_y + 1
    } else if tail_y > head_y {
        tail_y - 1
    } else {
        tail_y
    };

    return (new_x, new_y);
}

fn main() {
    // Initialization: head and tail start on top of each other at the origin
    // (0,0), and the tail has visited that point
    let mut head_x = 0i32;
    let mut head_y = 0i32;
    let mut tail_x = 0i32;
    let mut tail_y = 0i32;
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert((tail_x, tail_y));

    let data: String = fs::read_to_string("input.txt").unwrap();
    for line in data.lines() {
        let mut elems = line.split_whitespace();
        let direction = elems.next().unwrap();
        let steps = elems.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..steps {
            match direction {
                "D" => head_y -= 1,
                "U" => head_y += 1,
                "L" => head_x -= 1,
                "R" => head_x += 1,
                _ => panic!("bad direction: {}", direction),
            }
            let (new_x, new_y) = move_tail(head_x, head_y, tail_x, tail_y);
            tail_x = new_x;
            tail_y = new_y;
            tail_visited.insert((tail_x, tail_y));
        }
    }
    println!("tail visited {} locations", tail_visited.len());
}
