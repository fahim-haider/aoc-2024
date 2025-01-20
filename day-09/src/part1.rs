use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Block {
    is_free: bool,
    file_id: i32,
}

fn create_empty_block () -> Block {
    Block {is_free: true, file_id: -1}
}
fn create_filled_block (file_id: i32) -> Block {
    Block {is_free: false, file_id}
}

fn create_map (file: &str) -> (Vec<Block>, Vec<usize>) {
    let mut map: Vec<Block> = Vec::new();
    let mut free_map: Vec<usize> = Vec::new();
    let contents = fs::read_to_string(file).unwrap();
    let mut count_id = 0;

    for(index,ch) in contents.char_indices() {
        for _file in 0..ch.to_digit(10).unwrap() {
            if index % 2 == 0 { // If even index, file!
                map.push(create_filled_block(count_id));
            } else { // Else, info about free space
                map.push(create_empty_block());
                free_map.push(map.len()-1);
            }
        }
        if index % 2 == 0 { // Increment ID count iff even index!
            count_id += 1;
        }
    }
    (map,free_map)
}

fn disk_fragmenter (map: &mut Vec<Block>, free_map: &mut Vec<usize>) -> i64{
    let mut checksum: i64 = 0;
    let mut rev_index = map.len()-1;
    let mut forw_hole_index = 0;
    while forw_hole_index < free_map.len() 
    && rev_index > free_map[forw_hole_index] { // While free spaces exist
        if !map[rev_index].is_free {
            let hole = free_map[forw_hole_index];
            map.swap(rev_index, hole);
            //println!("Swapping! {:?}, {:?}", rev_index, hole);
            forw_hole_index += 1;
        }
        rev_index -= 1;
    }
    for (index, block) in map.iter().enumerate() {
        if block.file_id != -1 {
            checksum += (index as i32 * block.file_id) as i64;
        }
    }
    checksum
}

fn main() {
    let start_time = Instant::now();
    let (mut map, mut free_map) = create_map("src/input.in");
    //println!("Part 1: map: {:?}", free_map);
    let checksum = disk_fragmenter(&mut map, &mut free_map);
    let duration = start_time.elapsed();
    println!("Part 1: {:?}", checksum);
    //println!("Part 1: map: {:?}", map);
    println!("Time taken for Part 1 = {:?}", duration);
    let start_time = Instant::now();
    let duration = start_time.elapsed();
    //println!("Part 2: {:?}", sum);
    println!("Time taken for Part 2 = {:?}", duration);
}