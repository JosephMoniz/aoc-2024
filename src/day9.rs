fn expand_disk_map(input: &[u8]) -> Vec<Option<usize>> {
    let offsets: Vec<u8> = input.iter().map(|&c| c - b'0').collect::<Vec<_>>();
    let disk_size = offsets.iter().map(|&c| c as usize).sum();
    let mut disk_map = vec![None; disk_size];
    let mut disk_map_offset = 0;
    for (offset, &value) in offsets.iter().enumerate() {
        if offset % 2 == 0 {
            for _ in 0..value {
                disk_map[disk_map_offset] = Some(offset / 2);
                disk_map_offset += 1;
            }
        } else {
            disk_map_offset += value as usize;
        }
    }
    disk_map
}

fn compact_disk_map_with_fragmentation(disk_map: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut compacted_disk_map = vec![None; disk_map.len()];
    let mut left_offset = 0;
    let mut right_offset = disk_map.len() - 1;
    while left_offset <= right_offset {
        if disk_map[left_offset] == None {
            while disk_map[right_offset] == None && left_offset <= right_offset {
                right_offset -= 1;
            }
            compacted_disk_map[left_offset] = disk_map[right_offset];
            left_offset += 1;
            right_offset -= 1;
        } else {
            compacted_disk_map[left_offset] = disk_map[left_offset];
            left_offset += 1;
        }
    }
    compacted_disk_map
}

fn compact_disk_without_fragmentation(disk_map: &[Option<usize>]) -> Vec<Option<usize>> {
    let mut compacted_disk_map = Vec::from(disk_map);
    let mut first_free_slot = disk_map.iter().position(|n| matches!(n, None)).unwrap();
    let mut current_filled_slot = disk_map.iter().rposition(|n| matches!(n, Some(_))).unwrap();
    while first_free_slot < current_filled_slot {
        let current_file_id = compacted_disk_map[current_filled_slot].unwrap();
        let current_filled_length = (&compacted_disk_map[..current_filled_slot + 1])
            .iter()
            .rev()
            .take_while(|&fid| matches!(*fid, Some(n) if n == current_file_id))
            .count();
        let mut current_free_slot = first_free_slot;
        while current_free_slot < current_filled_slot {
            let current_free_length = (&compacted_disk_map[current_free_slot..])
                .iter()
                .take_while(|&fid| matches!(*fid, None))
                .count();
            if current_free_length >= current_filled_length {
                for i in 0..current_filled_length {
                    compacted_disk_map[current_free_slot + i] = Some(current_file_id);
                    compacted_disk_map[current_filled_slot - i] = None;
                }
                break;
            } else {
                let current_free_slot_end = current_free_slot + current_free_length;
                current_free_slot = (&compacted_disk_map[current_free_slot_end..])
                    .iter()
                    .position(|n| matches!(n, None))
                    .unwrap()
                    + current_free_slot_end;
            }
        }
        first_free_slot = compacted_disk_map
            .iter()
            .position(|n| matches!(n, None))
            .unwrap();
        current_filled_slot = (&compacted_disk_map
            [..current_filled_slot - current_filled_length + 1])
            .iter()
            .rposition(|&fid| matches!(fid, Some(_)))
            .unwrap();
    }
    compacted_disk_map
}

fn checksum_disk_map(disk_map: &[Option<usize>]) -> usize {
    disk_map.iter().enumerate().fold(0, |acc, (i, &c)| match c {
        None => acc,
        Some(file_id) => acc + (i * file_id),
    })
}

pub fn a(input: &[u8]) -> usize {
    let disk_map = expand_disk_map(input);
    let compacted_disk_map = compact_disk_map_with_fragmentation(&disk_map);
    checksum_disk_map(&compacted_disk_map)
}

pub fn b(input: &[u8]) -> usize {
    let disk_map = expand_disk_map(input);
    let compacted_disk_map = compact_disk_without_fragmentation(&disk_map);
    checksum_disk_map(&compacted_disk_map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9a() {
        // given ...
        let input_str = "2333133121414131402";
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 1928);
    }

    #[test]
    fn test_day9b() {
        // given ...
        let input_str = "2333133121414131402";
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 2858);
    }
}
