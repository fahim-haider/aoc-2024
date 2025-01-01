use itertools::Itertools;
use std::fs;
use ndarray::Array2;

// Contains all possible directions that the program can 'step'
const DIRECTIONS: &[(i32,i32)] = &[
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
fn find_steps(point: &(i32,i32), num_steps: usize) -> Vec<(i32,i32)> {
    let mut vec: Vec<(i32,i32)> = vec![(0,0)];
    for i in 1..num_steps {
        let (row,col) = *point;
        let step = (row * i as i32, col * i as i32);
        vec.push(step);
    }
    vec
}

// Finds the number of times that pattern occurs in the given matrix
fn count_num_pattern (matrix: &Array2<char>, pattern: &str) -> usize {
    // Find possible directions we should go
    let directions = DIRECTIONS.iter()
                        .map(|d| find_steps(d, pattern.len()))
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

// Find characters that are in an X pattern
fn count_X_pattern () {

}

fn main() {
    // Parse input file into a 2D array/matrix
    let content = parse_input_file("src/input.in");
    println!("Character at index[0][5] is: {}", content[[0,5]]);

    // Part 1: Find the number of instances of XMAS patterns
    let count = count_num_pattern(&content, "XMAS");
    println!("{count}");

    // Part 2: Find the number of instances that have MAS in as X pattern
    let count = count_X_pattern(&content, "XMAS");
    println!("{count}");
}
