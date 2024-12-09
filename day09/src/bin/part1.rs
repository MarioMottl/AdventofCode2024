use std::fs;

const INPUT_FILE: &str = "input.txt";

fn parse_disk_map(input: &str) -> Result<Vec<Option<usize>>, String> {
    let mut disk_map = Vec::new();
    let mut is_file = true;
    let mut file_id = 0;

    for c in input.chars() {
        let length = c.to_digit(10).ok_or_else(|| "Invalid digit".to_string())? as usize;
        for _ in 0..length {
            if is_file {
                disk_map.push(Some(file_id));
            } else {
                disk_map.push(None);
            }
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    Ok(disk_map)
}

fn get_free_blocks(disk_map: &[Option<usize>]) -> Vec<usize> {
    let mut free_blocks = Vec::new();
    for (pos, block) in disk_map.iter().enumerate() {
        if block.is_none() {
            free_blocks.push(pos);
        }
    }
    free_blocks.reverse();
    free_blocks
}

fn find_free_span(disk_map: &[Option<usize>], length: usize) -> Option<usize> {
    let mut current_length = 0;
    let mut start_pos = None;

    for (pos, block) in disk_map.iter().enumerate() {
        if block.is_none() {
            if current_length == 0 {
                start_pos = Some(pos);
            }
            current_length += 1;
            if current_length == length {
                return start_pos;
            }
        } else {
            current_length = 0;
            start_pos = None;
        }
    }

    None
}

fn compact_disk_map(mut disk_map: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut file_blocks: Vec<(usize, usize)> = Vec::new(); // (file_id, length)

    // Collect file blocks and their lengths
    let mut current_file_id = None;
    let mut current_length = 0;

    for &block in &disk_map {
        match block {
            Some(file_id) => {
                if Some(file_id) == current_file_id {
                    current_length += 1;
                } else {
                    if let Some(id) = current_file_id {
                        file_blocks.push((id, current_length));
                    }
                    current_file_id = Some(file_id);
                    current_length = 1;
                }
            }
            None => {
                if let Some(id) = current_file_id {
                    file_blocks.push((id, current_length));
                    current_file_id = None;
                    current_length = 0;
                }
            }
        }
    }
    if let Some(id) = current_file_id {
        file_blocks.push((id, current_length));
    }

    // Sort file blocks by file_id in descending order
    file_blocks.sort_by(|a, b| b.0.cmp(&a.0));

    // Move whole files to the leftmost span of free space blocks that could fit the file
    for (file_id, length) in file_blocks {
        if let Some(start_pos) = find_free_span(&disk_map, length) {
            let mut moved = false;
            for read_pos in (0..disk_map.len()).rev() {
                if disk_map[read_pos] == Some(file_id) {
                    disk_map[read_pos] = None;
                    disk_map[start_pos + length - 1] = Some(file_id);
                    length -= 1;
                    if length == 0 {
                        moved = true;
                        break;
                    }
                }
            }
            if moved {
                print_disk_map(&disk_map);
            }
        }
    }

    disk_map
}

fn calculate_checksum(disk_map: &[Option<usize>]) -> usize {
    disk_map
        .iter()
        .enumerate()
        .filter_map(|(pos, &file_id)| file_id.map(|id| pos * id))
        .sum()
}

fn print_disk_map(disk_map: &[Option<usize>]) {
    let map_str: String = disk_map
        .iter()
        .map(|&block| match block {
            Some(id) => std::char::from_digit(id as u32, 10).unwrap(),
            None => '.',
        })
        .collect();
    println!("{}", map_str);
}

fn main() {
    match fs::read_to_string(INPUT_FILE) {
        Ok(input) => match parse_disk_map(&input.trim()) {
            Ok(disk_map) => {
                let compacted_map = compact_disk_map(disk_map);
                let checksum = calculate_checksum(&compacted_map);
                println!("Filesystem checksum: {}", checksum);
            }
            Err(e) => eprintln!("Failed to parse disk map: {}", e),
        },
        Err(e) => eprintln!("Failed to read input file: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "2333133121414131402";
        let disk_map = parse_disk_map(input).unwrap();
        let compacted_map = compact_disk_map(disk_map);
        let checksum = calculate_checksum(&compacted_map);
        assert_eq!(checksum, 2858);
    }
}
