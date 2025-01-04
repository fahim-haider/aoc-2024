use std::collections::HashSet;
use std::fs;
use std::time::Instant;

// Guard moves upwards initially
const MOVE_UPWARDS: (i32, i32) = (-1, 0);

const MAP_SIZE: usize = 130;

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

fn count_guard_positions (map: &mut [[char; MAP_SIZE]; MAP_SIZE], 
                    guard: &mut Guard, obs_coord: (i32,i32)) -> (bool,HashSet<(i32,i32)>) {
    let mut turn_points: Vec<((i32,i32),(i32,i32))> = vec![];
    let mut step_pos: HashSet<(i32,i32)> = HashSet::new();
    let mut is_loop: bool = false;
    step_pos.insert(guard.pos);
    if obs_coord.0 >= 0 {
        map[obs_coord.0 as usize][obs_coord.1 as usize] = '#';
    }
    // Keep going until we leave the map!
    while guard.pos.0 + guard.dir.0 < map.len() as i32 
        && guard.pos.0 + guard.dir.0 >= 0 
        && guard.pos.1 + guard.dir.1 < map.len() as i32 
        && guard.pos.1 + guard.dir.1 >= 0 {
            let next_cell = map[(guard.pos.0+guard.dir.0) as usize]
                                [(guard.pos.1+guard.dir.1) as usize];
            //println!("next_cell {:?}", next_cell);
            // If obstacle infront of guard, turn right
            if next_cell == '#' {
                guard.dir = turn_right(guard.dir);
                if !turn_points.contains(&(guard.pos,guard.dir)) {
                    turn_points.push((guard.pos,guard.dir));
                } else {
                    //println!("Loop detected at {:?}", guard.pos);
                    is_loop = true;
                    if obs_coord.0 >= 0 {
                        map[obs_coord.0 as usize][obs_coord.1 as usize] = '.';
                    }
                    return (is_loop,step_pos);
                }
            } else {
                guard.pos.0 += guard.dir.0;
                guard.pos.1 += guard.dir.1;
                step_pos.insert(guard.pos);
            }
    }
    if obs_coord.0 >= 0 {
        map[obs_coord.0 as usize][obs_coord.1 as usize] = '.';
    }
    (is_loop,step_pos)
}

fn time_to_brute_force (mut base_map: &mut [[char; MAP_SIZE]; MAP_SIZE], 
                base_guard: &mut Guard, step_pos: &mut HashSet<(i32,i32)>) -> usize {
    step_pos.iter().filter(|a| {
                            if **a == base_guard.pos {
                                return false;
                            }
                            let mut guard = base_guard.clone();
                            let res = 
                                count_guard_positions(&mut base_map, &mut guard, **a);
                            //println!("result of {:?}: {:?}, {:?}", a, res.0,res.1.len());
                            res.0
                }).count()
}

fn main() {
    let start_time = Instant::now();
    let (mut map, guard) = create_map("src/input.in");
    let (_is_loop,mut step_pos) = 
        count_guard_positions(&mut map, &mut guard.clone(), (-1,-1));
    let duration = start_time.elapsed();
    println!("Part 1: {:?}, {:?}", _is_loop, step_pos.len());
    println!("Time taken for Part 1 = {:?}", duration);
    let start_time = Instant::now();
    let count = 
        time_to_brute_force(&mut map, &mut guard.clone(),&mut step_pos);
    let duration = start_time.elapsed();
    println!("Part 2: {:?}", count);
    println!("Time taken for Part 2 = {:?}", duration);
}
/*
Very very difficult day!
    - Learned alot about enums, structs again
    - Needed lots of hints like:
        - using HashSet to store all unique points
        - checking corner case where multiple turns are required
        - ensuring that initial spot of guard is included in hashset
    - Worked alot with tuples which helped greatly
    RESULTS:
        Part 1: 5.98 ms to create the array and parse through the guard's steps
        Part 2: 10.88 s(!) to find all possible obstruction locations
*/