use std::fs;
use grid::Grid;

fn parse_input_file(input_file: &str) -> Grid<char> {
    let contents = fs::read_to_string(input_file).unwrap();

    Grid::parse(contents)
    /*contents.lines()
            .map(|st| st.chars().collect())
            .collect::<Vec<_>>()*/
}

fn search_matrix (rows: usize, columns: usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for row in 0..rows {
        for column in 0..columns {
            if matrix[row][column] == 'X' {
                search_diagonally(row, column, matrix, &mut count);
                search_vertically(row, column, matrix, &mut count);
                search_horizontally(row, column, matrix, &mut count);
            }
        }
    }
    count
}

fn search_diagonally(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &mut i32) {
    if row <= 2 && column <= 2 {
        
    } else if column <= 2 {
        if (matrix[row][column], matrix[row+1][column+1], matrix[row+2][column+2], matrix[row+3][column+3]) == ('X','M','A','S') {
            *count += 1;
        }
    } else if row >= matrix.len() - 2 {

    } else if column >= matrix.len() - 2 {

    } else {

    }
}

fn search_vertically(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &mut i32) {

}

fn search_horizontally(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &mut i32) {

}

fn main() {
    let content = parse_input_file("src/input.in");
    println!("{}", content[0][5]);
}
