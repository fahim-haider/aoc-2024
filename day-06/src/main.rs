use std::fs;
use std::time::Instant;

// Guard moves upwards initially
const MOVE_UPWARDS: (i32, i32) = (-1, 0);

const MAP_SIZE: i32 = 130;

// Turn the guard right by swapping and negating vector elements
fn turn_right(prev_r: i32, prev_c: i32) -> (i32, i32) {
    (prev_c*-1, prev_r * -1)
}

fn create_map (file: &str) -> [[char; 130]; 130] {
    let contents = fs::read_to_string(file).unwrap();

    let mut map = [['.'; 130]; 130];

    for (r, line) in contents.split('\n').enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map[r][c] = ch;
        }
    }
    map
}

fn main() {
    let start = Instant::now();
    let map = create_map("src/input.in");
    let duration = start.elapsed();
    println!("Time taken = {:?}", duration)
}
