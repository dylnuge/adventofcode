// AoC 2022 Day 12

use std::collections::{HashSet, VecDeque};
use std::fs;

pub struct GraphPath {
    pub position: (usize, usize),
    pub trail: Vec<(usize, usize)>,
}

fn bfs_enqueue(
    visits: &mut VecDeque<GraphPath>,
    visited: &mut HashSet<(usize, usize)>,
    heightmap: &Vec<Vec<u8>>,
    curr_node: &mut GraphPath,
    new_x: usize,
    new_y: usize,
) {
    let new_pos = (new_x, new_y);

    // If we've *ever* visited a node before, we know there's a faster path to
    // it; we don't need to rescan it.
    if visited.contains(&new_pos) {
        return;
    }

    let (old_x, old_y) = curr_node.position;
    let old_height = heightmap[old_y][old_x];
    let new_height = heightmap[new_y][new_x];
    // For part 2 we're backtracking, so reversed from Part 1
    let height_diff = (old_height as isize) - (new_height as isize);
    // Visit a node iff it's at most 1 higher than the current node and hasn't
    // yet been visited
    if height_diff <= 1 && !curr_node.trail.contains(&new_pos) {
        let mut new_trail = curr_node.trail.clone();
        // We put the node we're stepping off into the trail
        new_trail.push(curr_node.position);
        let new_node = GraphPath {
            position: new_pos,
            trail: new_trail,
        };
        visits.push_back(new_node);
        visited.insert(new_pos);
    }
}

fn main() {
    let data: String = fs::read_to_string("input.txt").unwrap();
    let rows: usize = data.lines().count();
    let cols: usize = data.lines().next().unwrap().len();

    // heights range from 0 ('a') to 25 ('z') which can be contained in a u8
    let mut heightmap = vec![vec![0u8; cols]; rows];
    // These are (x, y) positions instead of (row#, col#), which makes them
    // inverted from the heightmap coordinates.
    let mut start_pos: (usize, usize) = (0, 0);

    // Build the heightmap and find the start and end points
    for (row, line) in data.lines().enumerate() {
        for (col, value) in line.chars().enumerate() {
            let char_val: u8 = value as u8;
            if char_val >= 'a' as u8 && char_val <= 'z' as u8 {
                heightmap[row][col] = char_val - ('a' as u8);
            } else if char_val == 'S' as u8 {
                heightmap[row][col] = 0;
            } else if char_val == 'E' as u8 {
                heightmap[row][col] = 25;
                start_pos = (col, row);
            } else {
                panic!("Bad character: {}", char_val as char);
            }
        }
    }

    // Breath first search to find the exit
    let mut visits = VecDeque::from([GraphPath {
        position: start_pos,
        trail: Vec::new(),
    }]);
    let mut visited = HashSet::<(usize, usize)>::new();

    loop {
        if visits.is_empty() {
            panic!(
                "Exhausted graph without ever hitting end point, maze may be
                unsolvable"
            );
        }

        let mut curr_node = visits.pop_front().unwrap();

        // If we found the end, the number of positions visited (excluding the
        // end) is the number of steps we took to get there
        let (curr_x, curr_y) = curr_node.position;
        let curr_height = heightmap[curr_y][curr_x];
        if curr_height == 0 {
            println!("Solution found in {} steps!", curr_node.trail.len());
            break;
        }

        // "Step" in all four directions if allowed (not too high from current
        // position, not outside the heightmap, not already visited on this
        // path)
        if curr_x + 1 < cols {
            bfs_enqueue(
                &mut visits,
                &mut visited,
                &heightmap,
                &mut curr_node,
                curr_x + 1,
                curr_y,
            );
        }
        if curr_x != 0 {
            bfs_enqueue(
                &mut visits,
                &mut visited,
                &heightmap,
                &mut curr_node,
                curr_x - 1,
                curr_y,
            );
        }
        if curr_y + 1 < rows {
            bfs_enqueue(
                &mut visits,
                &mut visited,
                &heightmap,
                &mut curr_node,
                curr_x,
                curr_y + 1,
            );
        }
        if curr_y != 0 {
            bfs_enqueue(
                &mut visits,
                &mut visited,
                &heightmap,
                &mut curr_node,
                curr_x,
                curr_y - 1,
            );
        }
    }
}
