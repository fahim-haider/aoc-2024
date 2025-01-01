use std::fs;

fn parse_input_file(input_file: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(input_file).unwrap();
    contents.lines()
            .map(|st| st.chars().collect())
            .collect::<Vec<_>>()
}

fn search_matrix (rows: usize, columns: usize, matrix: &Vec<Vec<char>>) -> i32 {
    let mut count: i32 = 0;
    for row in 0..rows {
        for column in 0..columns {
            if matrix[row][column] == 'X' {
                search_diagonally(row, column, matrix, &count);
                search_vertically(row, column, matrix, &count);
                search_horizontally(row, column, matrix, &count);
            }
        }
    }
    count
}

fn search_diagonally(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &i32) {
    if row <= 2 {
        for element in 0..matrix.len() {
            if matrix[row][element] == 'X' {
                
            }
        }
    } else if column <= 2 {

    } else if row >= matrix.len() - 2 {

    } else if column >= matrix.len() - 2 {

    } else {

    }
}

fn search_vertically(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &i32) {

}

fn search_horizontally(row: usize, column: usize, matrix: &Vec<Vec<char>>, count: &i32) {

}

fn main() {
    let content = parse_input_file("src/input.in");
    println!("{}", content[0][5]);
}
