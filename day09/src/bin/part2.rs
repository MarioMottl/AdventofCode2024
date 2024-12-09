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

fn find_free_spans(disk_map: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut free_spans = Vec::new();
    let mut current_length = 0;
    let mut start_pos = None;

    for (pos, block) in disk_map.iter().enumerate() {
        if block.is_none() {
            if current_length == 0 {
                start_pos = Some(pos);
            }
            current_length += 1;
        } else {
            if let Some(start) = start_pos {
                free_spans.push((start, current_length));
            }
            current_length = 0;
            start_pos = None;
        }
    }

    // Add the last free span if it exists
    if let Some(start) = start_pos {
        free_spans.push((start, current_length));
    }

    free_spans
}

fn get_file_blocks(disk_map: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut file_blocks: Vec<(usize, usize)> = Vec::new(); // (file_id, length)
    let mut current_file_id = None;
    let mut current_length = 0;

    for &block in disk_map {
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

    file_blocks.reverse();
    file_blocks
}

fn get_file_by_size(file_blocks: &[(usize, usize)], size: usize) -> Option<(usize, usize)> {
    file_blocks
        .iter()
        .find(|&&(_, file_size)| file_size == size)
        .cloned()
}

fn compact_disk_map(mut disk_map: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let free_spans = find_free_spans(&disk_map);
    let mut file_blocks = get_file_blocks(&disk_map);

    for (mut free_start, mut free_length) in free_spans {
        // Sort file_blocks by file_id in descending order
        file_blocks.sort_by(|a, b| b.0.cmp(&a.0));

        while free_length > 0 {
            if let Some((file_id, file_size)) = file_blocks
                .iter_mut()
                .filter(|&&mut (_, size)| size <= free_length && size > 0)
                .max_by_key(|&&mut (id, _)| id)
                .cloned()
            {
                for pos in free_start..free_start + file_size {
                    disk_map[pos] = Some(file_id);
                }

                // Set the moved file blocks to None in the original positions
                let mut remaining = file_size;
                for read_pos in (0..disk_map.len()).rev() {
                    if disk_map[read_pos] == Some(file_id) {
                        disk_map[read_pos] = None;
                        remaining -= 1;
                        if remaining == 0 {
                            break;
                        }
                    }
                }

                // Remove the file from file_blocks
                let file_pos = file_blocks
                    .iter()
                    .position(|&(id, _)| id == file_id)
                    .unwrap();
                file_blocks.remove(file_pos);

                // Update the free_length and free_start for the next iteration
                free_length -= file_size;
                free_start += file_size;
            } else {
                // No file fits in the remaining free space
                break;
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
        assert_eq!(checksum, 1928);
    }
}
