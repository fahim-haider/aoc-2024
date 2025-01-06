use std::fs;
use std::time::Instant;
use std::collections::{HashSet,HashMap};

const MAP_SIZE: usize = 50;

// Represents an antenna point in the map
#[derive(Clone, Debug)]
struct AntennaPoint {
    r: i32,
    c: i32,
}
impl AntennaPoint {
    fn get_anti_nodes (&mut self, p2: &mut AntennaPoint) -> (AntiNode,AntiNode) {
        let displacement = (p2.r - self.r, p2.c - self.c);
        
        let antinode1 = AntiNode::new(self.r + 2*displacement.0, 
                                                self.c + 2*displacement.1);
        let antinode2 = AntiNode::new(self.r - displacement.0, 
                                                self.c - displacement.1);
        (antinode1, antinode2)
    }
}

// Represents an antinode position
#[derive(Clone, Debug)]
#[derive(Eq, Hash, PartialEq)]
struct AntiNode {
    r: i32,
    c: i32,
    is_valid: bool
}
impl AntiNode {
    // Logic to check if antinode is valid
    fn new(r: i32, c: i32) -> AntiNode {
        let mut is_valid = false;
        if r >= 0 && r < MAP_SIZE as i32 
        && c >= 0 && c < MAP_SIZE as i32 {
            is_valid = true;
        }
        AntiNode {r, c, is_valid}
    }
}

// TODO: complete this using recursion?!?!?!
fn get_resonant_anti_nodes (p1: &AntennaPoint, p2: &AntennaPoint, set: &mut HashSet<AntiNode>) {
    let distance = (p2.r - p1.r, p2.c - p1.c);
    // holds displacement in one direction... will be incremented in a loop
    let mut displacement1 = (p1.r + 2*distance.0, p1.c + 2*distance.1);
    // holds displacement in the other direction... will be incremented in a loop
    let mut displacement2 = (p1.r - distance.0, p1.c - distance.1);
    // Increment displacement1 in steps of the distance between points
    while displacement1.0 < MAP_SIZE as i32 && displacement1.1 < MAP_SIZE as i32
    && displacement1.0 >= 0 as i32 && displacement1.1 >= 0 as i32  {
        let antinode1 = AntiNode::new(displacement1.0, 
            displacement1.1);
        set.insert(antinode1);
        displacement1.0 += distance.0;
        displacement1.1 += distance.1;
    }
    // Increments displacement2 similarly
    while displacement2.0 < MAP_SIZE as i32 && displacement2.1 < MAP_SIZE as i32
    && displacement2.0 >= 0 as i32 && displacement2.1 >= 0 as i32  {
        let antinode1 = AntiNode::new(displacement2.0, 
            displacement2.1);
        set.insert(antinode1);
        displacement2.0 -= distance.0;
        displacement2.1 -= distance.1;
    }
}

// Create array of map and store all locations of each antenna type
fn create_map (file: &str) -> ([[char; MAP_SIZE]; MAP_SIZE], HashMap<char, Vec<AntennaPoint>>) {
    let contents = fs::read_to_string(file).unwrap();
    let mut map = [['.'; MAP_SIZE]; MAP_SIZE];
    let mut hash: HashMap<char, Vec<AntennaPoint>> = HashMap::new();

    for (r, line) in contents.split_whitespace().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                map[r][c] = ch;
                match hash.get_mut(&ch) {
                    Some(vec) => {
                        vec.push(AntennaPoint {r: r as i32,c: c as i32});
                    },
                    None => {
                        hash.insert(ch, 
                                vec![AntennaPoint{r: r as i32,c: c as i32}]);
                    },
                };
            }
        }
    }
    (map, hash)
}

// Finds all antinodes based on critera from part 1 (only two antinodes per pair of antennas)
fn find_antinodes (mut hash: HashMap<char, Vec<AntennaPoint>>) -> usize {
    let mut set: HashSet<AntiNode> = HashSet::new();
    for vec in hash.values_mut() {
        for i in 0..vec.len() {
            for j in i..(vec.len()-1) {
                let (left,right) = vec.split_at_mut(i+1);
                let curr_point = &mut left[i];
                let next_point = &mut right[j-i];
                let (an1, an2) = curr_point.get_anti_nodes(next_point);
                if an1.is_valid == true {
                    set.insert(an1);
                }
                if an2.is_valid == true {
                    set.insert(an2);
                }
            }
        }
    }
    set.len()
}

// Finds all antinodes including the antennas themselves AND resonant harmonics
fn find_resonant_antinodes (mut hash: HashMap<char, Vec<AntennaPoint>>) -> usize {
    let mut set: HashSet<AntiNode> = HashSet::new();
    for vec in hash.values_mut() {
        for i in 0..vec.len() {
            for j in i..(vec.len()-1) {
                let (left,right) = vec.split_at_mut(i+1);
                let curr_point = &mut left[i];
                let next_point = &mut right[j-i];
                set.insert(AntiNode {r: curr_point.r, c: curr_point.c, is_valid: true});
                set.insert(AntiNode {r: next_point.r, c: next_point.c, is_valid: true});
                get_resonant_anti_nodes(curr_point, next_point, &mut set);
            }
        }
    }
    set.len()
}

fn main() {
    let start_time = Instant::now();
    let (_map, hash) = create_map("src/input.in");
    let sum = find_antinodes(hash.clone());
    let duration = start_time.elapsed();
    println!("Part 1: {:?}", sum);
    println!("Time taken for Part 1 = {:?}", duration);
    let start_time = Instant::now();
    let sum = find_resonant_antinodes(hash);
    let duration = start_time.elapsed();
    println!("Part 2: {:?}", sum);
    println!("Time taken for Part 2 = {:?}", duration);
}
/*
RESULTS:
Part 1: 265, solved in 1.12 ms!!!
Part 2: 962, solved in 1.49 ms!
*/