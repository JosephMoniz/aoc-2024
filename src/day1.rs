fn read_and_parse_and_sort_lists() -> (Vec<usize>, Vec<usize>) {
    // grab the input and validate it
    let input = include_bytes!("../inputs/day1.txt");
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

pub fn a() {
    // read, parse, and sort the lists
    let (left, right) = read_and_parse_and_sort_lists();

    // zip the two lists together and calculate the difference between each pair and sum them up
    let mut total_difference = 0;
    for (l, r) in left.iter().zip(right.iter()) {
        let difference = if l > r { l - r } else { r - l };
        total_difference += difference;
    }

    println!("Day 1a: {}", total_difference);
}

pub fn b() {
    // read, parse, and sort the lists
    let (left, right) = read_and_parse_and_sort_lists();

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
            let similarity = left_current * right_count;
            total_similarity += similarity;
            left_offset += 1;
            while left_offset < left.len() && left[left_offset] == left_current {
                left_offset += 1;
            }
        } else if left[left_offset] < right[right_offset] {
            left_offset += 1;
        } else {
            right_offset += 1;
        }
    }

    println!("Day 1b: {}", total_similarity);
}
