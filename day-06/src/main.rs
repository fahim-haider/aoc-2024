use std::fs;
use std::time::Instant;

// Guard moves upwards initially
const MOVE_UPWARDS: (i32, i32) = (-1, 0);

const MAP_SIZE: usize = 130;

// Turn the guard right by swapping and negating vector elements
fn turn_right(direction: (i32,i32)) -> (i32, i32) {
    (direction.1, direction.0 * -1)
}

fn create_map (file: &str) -> ([[char; MAP_SIZE]; MAP_SIZE], (i32,i32)) {
    let contents = fs::read_to_string(file).unwrap();

    let mut map = [['.'; MAP_SIZE]; MAP_SIZE];
    let mut start: (i32, i32)= (0,0);

    for (r, line) in contents.split_whitespace().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map[r][c] = ch;
            if ch == '^' {
                start = (r as i32,c as i32);
            }
        }
    }
    (map, start)
}

fn count_guard_positions (map: &mut [[char; MAP_SIZE]; MAP_SIZE], start: (i32,i32)) -> usize {
    let mut count: usize = 0;
    let mut point = start;
    let mut direction = MOVE_UPWARDS;
    // Keep going until we leave the map!
    while point.0 + direction.0 < map.len() as i32 && point.0 + direction.0 >= 0 
        && point.1 + direction.1 < map.len() as i32 && point.1 + direction.1 >= 0 {
            // If obstacle infront of guard, turn right
            if map[(point.0+direction.0) as usize]
            [(point.1+direction.1) as usize] == '#' {
                direction = turn_right(direction);
            }
            if map[(point.0+direction.0) as usize]
            [(point.1+direction.1) as usize] != 'X' {
                count += 1;
            }
            map[(point.0) as usize][(point.1) as usize] = 'X';
            point.0 += direction.0;
            point.1 += direction.1;
            map[(point.0) as usize][(point.1) as usize] = '^';
    }
    count += 1; // For the last position in the array before it steps out
    count
}

fn main() {
    let start_time = Instant::now();
    let (mut map, start_point) = create_map("src/input.in");
    let count = count_guard_positions(&mut map, start_point);
    let duration = start_time.elapsed();
    println!("Part 1: {count}");
    println!("Time taken for Part 1 = {:?}", duration)
}
