use crate::common::slice_to_usize;
use std::collections::HashMap;

fn parse_input(input: &[u8]) -> Vec<usize> {
    input.split(|&c| c == b' ').map(slice_to_usize).collect()
}

fn count_digits(stone: usize) -> usize {
    let mut remaining = stone;
    let mut count = 0;
    while remaining != 0 {
        remaining /= 10;
        count += 1;
    }
    count
}

fn split_digits(stone: usize, digit_count: usize) -> (usize, usize) {
    let split_at = (digit_count + 1) / 2;
    let divisor = 10_usize.pow((digit_count - split_at) as u32);
    let first = stone / divisor;
    let second = stone % divisor;
    (first, second)
}

fn blink(stones: Vec<usize>) -> Vec<usize> {
    let mut stones_after_blink = Vec::with_capacity(stones.len() * 2);
    for &stone in stones.iter() {
        if stone == 0 {
            stones_after_blink.push(1);
        } else {
            let digit_count = count_digits(stone);
            if digit_count % 2 == 0 {
                let (first, second) = split_digits(stone, digit_count);
                stones_after_blink.push(first);
                stones_after_blink.push(second);
            } else {
                stones_after_blink.push(stone * 2024);
            }
        }
    }
    stones_after_blink
}

fn blink_memoized(
    stone: usize,
    remaining: usize,
    memos: &mut HashMap<(usize, usize), usize>,
) -> usize {
    if remaining == 0 {
        1
    } else {
        if let Some(&result) = memos.get(&(stone, remaining)) {
            result
        } else {
            let result = if stone == 0 {
                blink_memoized(1, remaining - 1, memos)
            } else {
                let digit_count = count_digits(stone);
                if digit_count % 2 == 0 {
                    let (first, second) = split_digits(stone, digit_count);
                    blink_memoized(first, remaining - 1, memos)
                        + blink_memoized(second, remaining - 1, memos)
                } else {
                    blink_memoized(stone * 2024, remaining - 1, memos)
                }
            };
            memos.insert((stone, remaining), result);
            result
        }
    }
}

pub fn a(input: &[u8]) -> usize {
    let mut stones = parse_input(input);
    for _ in 0..25 {
        stones = blink(stones);
    }
    stones.len()
}

pub fn b(input: &[u8]) -> usize {
    let mut memos = HashMap::new();
    parse_input(input)
        .into_iter()
        .map(|stone| blink_memoized(stone, 75, &mut memos))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day11a() {
        // given ...
        let input_str = "125 17";
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 55312);
    }

    #[test]
    fn test_day11b() {
        // given ...
        let input_str = "125 17";
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 65601038650482);
    }
}
