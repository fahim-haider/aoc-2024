use itertools::Itertools;
use std::fs;
use ndarray::Array2;

// Contains all possible directions that the program can 'step'
const XMAS_DIRECTIONS: &[(i32,i32)] = &[
    // Straight
    (0,-1),
    (0,1),
    (1,0),
    (-1,0),

    // Diagonal
    (-1,-1),
    (-1,1),
    (1,-1),
    (1,1)
];

// Contains all possible directions that the X pattern can extend to
const X_DIRECTIONS: &[(i32,i32)] = &[
    (-1,1),
    (1,1)
];

fn parse_input_file(input_file: &str) -> Array2<char> {
    let contents = fs::read_to_string(input_file).unwrap();

    let vec = contents.lines()
                        .map(|st| st.chars().collect::<Vec<_>>())
                        .collect::<Vec<_>>();

    let row_size = vec.len();
    let col_size = vec[0].len();

    let flat_vec = vec.into_iter().flatten().collect::<Vec<_>>();
    Array2::<char>::from_shape_vec((row_size, col_size), flat_vec).unwrap()
}

// Find all possible steps from the current point in the matrix
fn find_steps(point: &(i32,i32), num_steps: i32) -> Vec<(i32,i32)> {
    let mut vec: Vec<(i32,i32)> = vec![(0,0)];
    let (row,col) = *point;
    for i in 1..num_steps {
        let step = (row * i, col * i);
        vec.push(step);
    }
    vec
}

// Finds the number of times that pattern occurs in the given matrix
fn count_num_pattern (matrix: &Array2<char>, pattern: &str) -> usize {
    // Find possible directions we could go
    let directions = XMAS_DIRECTIONS.iter()
                        .map(|d| find_steps(d, pattern.len() as i32))
                        .collect::<Vec<_>>();

    let mut count: usize = 0;
    let (row_size, col_size) = (matrix.shape()[0],matrix.shape()[1]);
    for (row, col) in (0..row_size).cartesian_product(0..col_size) {
        // Find valid positions for this element
        let valid_directons = directions.iter()
                                .map(|d| 
                                    d.iter()
                                    .map(|(r, c)| (row as i32 + r, col as i32 + c))) 
                                .filter(|d| 
                                    d.clone().all(|(r, c)| r >= 0 && r < row_size as i32
                                        && c >= 0 && c < col_size as i32))
                                .map(|d| d.collect::<Vec<_>>())
                                .collect::<Vec<_>>();

        // Find the strings that the valid directions contain
        let strings = valid_directons.iter()
                                .map(|d| 
                                    d.iter().map(|(r,c)| matrix[(*r as usize,*c as usize)]))
                                .map(|c| c.collect::<Vec<_>>())
                                .collect::<Vec<_>>();
        
        // Find the strings that match the pattern and count them
        count += strings.iter().filter(|str| **str == pattern.chars().collect::<Vec<_>>()).count();
    }
    count
}

// Find all possible steps from the current point in the matrix
fn find_mas_steps(point: &(i32,i32), num_steps: i32) -> Vec<(i32,i32)> {
    let mut vec: Vec<(i32,i32)> = vec![];
    let (row,col) = *point;
    for i in -1..=num_steps {
        let step = (row * i, col * i);
        vec.push(step);
    }
    vec
}

// Find characters that are in an X pattern
fn count_mas_pattern (matrix: &Array2<char>, pattern: &str) -> usize {
    // Find possible directions we could go
    let directions = X_DIRECTIONS.iter()
            .map(|d| find_mas_steps(d, 1))
            .collect::<Vec<_>>();

    let mut count: usize = 0;
    let (row_size, col_size) = (matrix.shape()[0],matrix.shape()[1]);
    for (row, col) in (0..row_size).cartesian_product(0..col_size) {
        // Find valid positions for this element
        let valid_directons = directions.iter()
                                .map(|d| 
                                    d.iter()
                                    .map(|(r, c)| (row as i32 + r, col as i32 + c))) 
                                .filter(|d| 
                                    d.clone().all(|(r, c)| r >= 0 && r < row_size as i32
                                        && c >= 0 && c < col_size as i32))
                                .map(|d| d.collect::<Vec<_>>())
                                .collect::<Vec<_>>();

        // Find the strings that the valid directions contain
        let strings = valid_directons.iter()
                                .map(|d| 
                                    d.iter().map(|(r,c)| matrix[(*r as usize,*c as usize)]))
                                .map(|c| c.collect::<Vec<_>>())
                                .collect::<Vec<_>>();
        
        // Find the strings that match the pattern and count them
        if (strings.iter().all(|str| 
            *str == pattern.chars().collect::<Vec<_>>()
            || *str == pattern.chars().rev().collect::<Vec<_>>())) && (strings.len() > 0) { 
                count += 1; 
        }
    }
    count
}

fn main() {
    // Parse input file into a 2D array/matrix
    let content = parse_input_file("src/input.in");
    println!("Character at index[0][1] is: {}", content[[0,1]]);

    // Part 1: Find the number of instances of XMAS patterns
    let count = count_num_pattern(&content, "XMAS");
    println!("{count}");

    // Part 2: Find the number of instances that have MAS in as X pattern
    let count = count_mas_pattern(&content, "MAS");
    println!("{count}");
}
/*
    - Not considering that all returns true if strings is empty took hours off my life lool
    - Learned allot about grids and how they are more efficient than 2D vectors
    - More practice on map, filter, all!
    - Learned to use the directions constant rather than hardcoding the possible points
*/