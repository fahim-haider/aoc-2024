use std::fs;
use std::time::Instant;

#[derive(Clone, Debug)]
struct Partition {
    blocks: Vec<i32>,
    size: u32,
    start_index: u32
}

impl Partition {
    fn is_empty (self) -> bool {
        self.blocks.iter().all(|a| *a == -1)
    }
}

fn create_empty_block (size: u32, start_index: u32) -> Partition {
    Partition {blocks: vec![-1; size as usize], size, start_index}
}
fn create_filled_block (file_id: i32, size: u32, start_index: u32) -> Partition {
    Partition {blocks: vec![file_id; size as usize], size, start_index}
}

fn create_map (file: &str) -> (Vec<Partition>, Vec<usize>) {
    let mut map: Vec<Partition> = Vec::new();
    let mut free_map: Vec<usize> = Vec::new();
    let contents = fs::read_to_string(file).unwrap();
    let mut count_id = 0;
    let mut count_index = 0;

    for(index,ch) in contents.char_indices() {
        let size = ch.to_digit(10).unwrap();
        if index % 2 == 0 { // If even index, file!
            map.push(create_filled_block(count_id, size, count_index));
            count_id += 1;
        } else { // Else, info about free space
            map.push(create_empty_block(size, count_index));
            free_map.push(map.len()-1);
        }
        count_index += size;
    }
    println!("{:?}", free_map);
    (map,free_map)
}

// assumes that the indices it receives are valid to swap (fragment size < hole size)
fn swap_fragments (map: &mut Vec<Partition>, free_map: &mut Vec<usize>, 
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

fn disk_fragmenter (map: &mut Vec<Partition>, free_map: &mut Vec<usize>) -> i64{
    let mut checksum: i64 = 0;
    let mut rev_index = map.len()-1;
    let mut forw_hole_index = 0;
    while forw_hole_index < free_map.len() 
    && rev_index > free_map[forw_hole_index] { // While free spaces exist
        if map[rev_index].blocks[0] == -1 {
            let hole = free_map[forw_hole_index];
            swap_fragments(map, free_map, rev_index, hole);
            //println!("Swapping! {:?}, {:?}", rev_index, hole);
            //map.swap(rev_index, hole);
            forw_hole_index += 1;
        }
        rev_index -= 1;
    }
    for (index, block) in map.iter().enumerate() {
        let file_id = block.blocks[0];
        if file_id != -1 {
            checksum += (index as i32 * file_id) as i64;
        }
    }
    checksum
}

fn main() {
    //let start_time = Instant::now();
    let (mut map, mut free_map) = create_map("src/input.in");
    println!("map: {:?}", map);
    //let checksum = disk_fragmenter(&mut map, &mut free_map);
    //let duration = start_time.elapsed();
    //println!("Part 1: {:?}", checksum);
    //println!("Part 1: map: {:?}", map);
    //println!("Time taken for Part 1 = {:?}", duration);
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