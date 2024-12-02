fn read_and_parse_and_sort_lists(input: &[u8]) -> (Vec<usize>, Vec<usize>) {
    // validate the input
    let maybe_last_char = input.last();
    if let None = maybe_last_char {
        panic!("Empty input");
    }
    if let Some(&n) = maybe_last_char {
        if n != b'\n' {
            panic!("Input not newline terminated");
        }
    }

    // calculate the exact length of the right and left list
    let list_size = input.len() / 14;

    // parse each line into two unordered lists
    let mut left = Vec::with_capacity(list_size);
    let mut right = Vec::with_capacity(list_size);
    let mut cursor = &input[..];
    while cursor.len() > 0 {
        // each number is 5 characters each with three spaces in between followed by a newline
        let ls = unsafe { std::str::from_utf8_unchecked(&cursor[..5]) };
        let ln: usize = ls.parse().unwrap();
        left.push(ln);
        let rn = unsafe { std::str::from_utf8_unchecked(&cursor[8..13]) };
        let r: usize = rn.parse().unwrap();
        right.push(r);
        cursor = &cursor[14..];
    }

    // sort the lists and return them
    left.sort();
    right.sort();
    (left, right)
}

pub fn a(input: &[u8]) -> usize {
    // read, parse, and sort the lists
    let (left, right) = read_and_parse_and_sort_lists(input);

    // zip the two lists together and calculate the difference between each pair and sum them up
    let mut total_difference = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let difference = if l > r { l - r } else { r - l };
        total_difference += difference;
    }

    total_difference
}

pub fn b(input: &[u8]) -> usize {
    // read, parse, and sort the lists
    let (left, right) = read_and_parse_and_sort_lists(input);

    // "crab-walk" through both sorted lists, counting occurrences of each number and
    // applying the similarity score calculation
    let mut left_offset = 0;
    let mut right_offset = 0;
    let mut total_similarity = 0;
    while left_offset < left.len() && right_offset < right.len() {
        let left_current = left[left_offset];
        if left_current == right[right_offset] {
            let right_start = right_offset;
            while right_offset < right.len() && left_current == right[right_offset] {
                right_offset += 1;
            }
            let right_end = right_offset;
            let right_count = right_end - right_start;
            let right_similarity = left_current * right_count;
            let left_start = left_offset;
            while left_offset < left.len() && left[left_offset] == left_current {
                left_offset += 1;
            }
            let left_end = left_offset;
            let left_count = left_end - left_start;
            total_similarity += right_similarity * left_count;
        } else if left[left_offset] < right[right_offset] {
            left_offset += 1;
        } else {
            right_offset += 1;
        }
    }

    total_similarity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day1a() {
        // given ...
        let input_str = vec![
            "30000   40000",
            "40000   30000",
            "20000   50000",
            "10000   30000",
            "30000   90000",
            "30000   30000",
            "",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 110000);
    }

    #[test]
    fn test_day1b() {
        // given ...
        let input_str = vec![
            "30000   40000",
            "40000   30000",
            "20000   50000",
            "10000   30000",
            "30000   90000",
            "30000   30000",
            "",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 310000);
    }
}
