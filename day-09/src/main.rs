use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Block {
    file_id: i32,
    position: usize
}

impl Block {
    fn calc_checksum (self) -> i32 {
        let checksum = self.file_id * (self.position as i32);
        checksum
    }
}

fn create_empty_block (position: usize) -> Block {
    Block {file_id: -1, position}
}
fn create_filled_block (file_id: i32, position: usize) -> Block {
    Block {file_id, position}
}

fn create_map (file: &str) -> (Vec<Vec<Block>>, Vec<usize>) {
    let mut map: Vec<Vec<Block>> = Vec::new();
    let mut free_map: Vec<usize> = Vec::new();
    let contents = fs::read_to_string(file).unwrap();
    let mut count_id = 0;
    let mut position = 0;

    for(index,ch) in contents.char_indices() {
        let size = ch.to_digit(10).unwrap();
        let mut temp: Vec<Block> = Vec::new();
        for i in 0..size {
            if index % 2 == 0 { // If even index, file!
                temp.push(create_filled_block(count_id, position));
            } else { // Else, info about free space
                temp.push(create_empty_block(position));
                free_map.push(map.len()-1);
            }
            position += 1;
        }
        map.push(temp);
        if index % 2 == 0 {
            count_id += 1;
        }
    }
    (map,free_map)
}

// assumes that the indices it receives are valid to swap (fragment size < hole size)
fn swap_fragments (map: &mut Vec<Vec<Block>>, free_map: &mut Vec<usize>, 
    fragment_index: usize, hole_index: usize) {
        if map[fragment_index].size == map[hole_index].size {
            map.swap(fragment_index, hole_index);
            free_map[hole_index] = fragment_index; 
        } else { // case: hole size and fragment size are not equal
            // swap...
            map.swap(fragment_index, hole_index);
            // create hole next to fragment index

            // shorten size of hole after swapping to match fragment size

            // update hole list


        }

}

fn disk_fragmenter (map: &mut Vec<Vec<Block>>, free_map: &mut Vec<usize>) -> i64{
    let mut checksum: i64 = 0;
    let mut rev_index = map.len()-1;
    let mut forw_hole_index = 0;
    while forw_hole_index < free_map.len() 
    && rev_index > free_map[forw_hole_index] { // While free spaces exist
        if map[rev_index].file_id == 1 {
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
    println!("Part 1: map: {:?}", map);
    //let checksum = disk_fragmenter(&mut map, &mut free_map);
    let duration = start_time.elapsed();
    //println!("Part 1: {:?}", checksum);
    //println!("Part 1: map: {:?}", map);
    println!("Time taken for Part 1 = {:?}", duration);
    let start_time = Instant::now();
    //let checksum = disk_fragmenter(&mut map, &mut free_map);
    let duration = start_time.elapsed();
    //println!("Part 2: {:?}", sum);
    println!("Time taken for Part 2 = {:?}", duration);
}
/*

Results:
Part 1: 6242766523059, solved in 7 ms
Part 2: 
*/