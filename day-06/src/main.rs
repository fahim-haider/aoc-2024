use std::fs;
use std::time::Instant;

// Guard moves upwards initially
const MOVE_UPWARDS: (i32, i32) = (-1, 0);

const MAP_SIZE: usize = 10;

#[derive(Clone, Debug)]
struct Guard {
    pos: (i32, i32),
    dir: (i32, i32)
}

// Turn the guard right by swapping and negating vector elements
fn turn_right(direction: (i32,i32)) -> (i32, i32) {
    (direction.1, -direction.0)
}

fn create_map (file: &str) -> ([[char; MAP_SIZE]; MAP_SIZE], Guard) {
    let contents = fs::read_to_string(file).unwrap();

    let mut map = [['.'; MAP_SIZE]; MAP_SIZE];
    let mut guard: Guard = Guard {pos: (0,0), dir: (MOVE_UPWARDS)};

    for (r, line) in contents.split_whitespace().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map[r][c] = ch;
            if ch == '^' {
                guard.pos = (r as i32, c as i32);
            }
        }
    }
    (map, guard)
}

fn count_guard_positions (map: &mut [[char; MAP_SIZE]; MAP_SIZE], guard: &mut Guard) -> usize {
    let mut count: usize = 0;

    // Keep going until we leave the map!
    while guard.pos.0 + guard.dir.0 < map.len() as i32 
        && guard.pos.0 + guard.dir.0 >= 0 
        && guard.pos.1 + guard.dir.1 < map.len() as i32 
        && guard.pos.1 + guard.dir.1 >= 0 {
            let next_cell = map[(guard.pos.0+guard.dir.0) as usize]
                                [(guard.pos.1+guard.dir.1) as usize];
            // If obstacle infront of guard, turn right
            if next_cell == '#' {
                guard.dir = turn_right(guard.dir);
            }
            // If next cell has not been traversed, count++
            if next_cell != 'X' {
                count += 1;
            }
            map[(guard.pos.0) as usize][(guard.pos.1) as usize] = 'X';
            guard.pos.0 += guard.dir.0;
            guard.pos.1 += guard.dir.1;
            map[(guard.pos.0) as usize][(guard.pos.1) as usize] = '^';
    }
    // For the last position in the array before the guard steps out
    map[(guard.pos.0) as usize][(guard.pos.1) as usize] = 'X';
    count += 1;
    count
}

fn main() {
    let start_time = Instant::now();
    let (mut map, mut start_point) = create_map("src/input.in");
    let count = count_guard_positions(&mut map, &mut start_point);
    let duration = start_time.elapsed();
    println!("Part 1: {count}");
    println!("Time taken for Part 1 = {:?}", duration);
    println!("The last row of the grid: {:?}", map[9]);
}
