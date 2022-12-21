// Advent of Code Day 9

use std::collections::HashSet;
use std::fs;

// Move knot2 towards knot1 based on the movement rules
fn move_knot(knot1_pos: (i32, i32), knot2_pos: (i32, i32)) -> (i32, i32) {
    let (knot1_x, knot1_y) = knot1_pos;
    let (knot2_x, knot2_y) = knot2_pos;

    let x_dist = (knot2_x - knot1_x).abs();
    let y_dist = (knot2_y - knot1_y).abs();
    if x_dist <= 1 && y_dist <= 1 {
        // knot2 is within 1 square of knot1 (including on top of it), doesn't
        // move
        return (knot2_x, knot2_y);
    }

    // Move a square left or right towards the knot1
    let new_x = if knot2_x < knot1_x {
        knot2_x + 1
    } else if knot2_x > knot1_x {
        knot2_x - 1
    } else {
        knot2_x
    };

    // Move a square up or down towards the knot1
    let new_y = if knot2_y < knot1_y {
        knot2_y + 1
    } else if knot2_y > knot1_y {
        knot2_y - 1
    } else {
        knot2_y
    };

    return (new_x, new_y);
}

fn main() {
    // Initialization: all knots start on top of each other at the origin (0,0),
    // and the tail has visited that point. To solve Part 1 instead of Part 2
    // with this code, all that needs to change is the size of the vec (from 10
    // to 2).
    let mut knots = vec![(0i32, 0i32); 10];
    let tail_idx = knots.len() - 1;
    let mut tail_visited: HashSet<(i32, i32)> = HashSet::new();
    tail_visited.insert(knots[tail_idx]);

    let data: String = fs::read_to_string("input.txt").unwrap();
    for line in data.lines() {
        let mut elems = line.split_whitespace();
        let direction = elems.next().unwrap();
        let steps = elems.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..steps {
            // Move the head knot
            let (mut head_x, mut head_y) = knots[0];
            match direction {
                "D" => head_y -= 1,
                "U" => head_y += 1,
                "L" => head_x -= 1,
                "R" => head_x += 1,
                _ => panic!("bad direction: {}", direction),
            }
            knots[0] = (head_x, head_y);

            // Move the remaining knots
            for knot in 1..knots.len() {
                knots[knot] = move_knot(knots[knot - 1], knots[knot])
            }
            tail_visited.insert(knots[tail_idx]);
        }
    }
    println!("tail visited {} locations", tail_visited.len());
}
