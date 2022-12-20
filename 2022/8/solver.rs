// Advent of Code 2022 Day 8

use std::fs;

fn main() {
    let data: String = fs::read_to_string("input.txt").unwrap();
    let rows: usize = data.lines().count();
    let cols: usize = data.lines().next().unwrap().len();

    // For a minor optimization, if there's not at least three rows and columns,
    // everything is on an edge and thus visible. This makes the code below
    // sound (i.e. we don't double count the edges if they're overlapping, and
    // can safely avoid the edges as a slight optimization without bounds
    // checks).
    if cols <= 2 || rows <= 2 {
        println!("Visible elems: {}", cols * rows);
        return;
    }

    // Build data arrays. vals are all single digits, so won't exceed a u8
    let mut values = vec![vec![0u8; cols]; rows];
    let mut visible = vec![vec![false; cols]; rows];

    // Parse the data into a 2D array of ints.
    for (row, line) in data.lines().enumerate() {
        for (col, value) in line.chars().enumerate() {
            values[row][col] = value.to_digit(10).unwrap() as u8;
        }
    }

    // Checking by direction instead of by cell is faster by a quadratic order:
    // O(n*m(n+m)) (2n^3 on even sides) for cell-by-cell check vs O(4*n*m) (4n^2
    // on even sides).
    let mut visible_count: usize = 0;
    // First & last row and col are all visible, mark in advance
    for col in 0..cols {
        visible[0][col] = true;
        visible[rows - 1][col] = true;
        visible_count += 2;
    }
    for row in 1..rows - 1 {
        visible[row][0] = true;
        visible[row][cols - 1] = true;
        visible_count += 2;
    }

    // Rows
    for row in 1..rows - 1 {
        // From left
        let mut largest_tree_left = values[row][0];
        for col in 1..cols - 1 {
            let tree = values[row][col];
            if tree > largest_tree_left {
                if !visible[row][col] {
                    visible[row][col] = true;
                    visible_count += 1;
                }
                largest_tree_left = tree;
            }
        }

        // From right
        let mut largest_tree_right = values[row][cols - 1];
        for col in (1..cols - 1).rev() {
            let tree = values[row][col];
            if tree > largest_tree_right {
                if !visible[row][col] {
                    visible[row][col] = true;
                    visible_count += 1;
                }
                largest_tree_right = tree;
            }
        }
    }

    // Cols
    for col in 1..cols - 1 {
        // From top
        let mut largest_tree_top = values[0][col];
        for row in 1..rows - 1 {
            let tree = values[row][col];
            if tree > largest_tree_top {
                if !visible[row][col] {
                    visible[row][col] = true;
                    visible_count += 1;
                }
                largest_tree_top = tree;
            }
        }

        // From right
        let mut largest_tree_bottom = values[rows - 1][col];
        for row in (1..rows - 1).rev() {
            let tree = values[row][col];
            if tree > largest_tree_bottom {
                if !visible[row][col] {
                    visible[row][col] = true;
                    visible_count += 1;
                }
                largest_tree_bottom = tree;
            }
        }
    }

    println!("Visible trees: {}", visible_count);
}
