fn slice_to_usize(slice: &[u8]) -> usize {
    let str = unsafe { std::str::from_utf8_unchecked(slice) };
    str.parse::<usize>().unwrap()
}

fn is_safe_decreasing(previous: usize, current: usize) -> bool {
    current < previous && current >= previous.saturating_sub(3)
}

fn is_safe_increasing(previous: usize, current: usize) -> bool {
    current > previous && current <= previous.saturating_add(3)
}

fn is_remainder_safe<'a, F>(
    mut previous: usize,
    levels: impl Iterator<Item = &'a [u8]>,
    test: F,
) -> bool
where
    F: Fn(usize, usize) -> bool,
{
    for level in levels {
        let current = slice_to_usize(level);
        if test(previous, current) {
            previous = current;
        } else {
            return false;
        }
    }
    true
}

pub fn a(input: &[u8]) -> usize {
    input
        .split(|&c| c == b'\n')
        .filter(|report| {
            let mut levels = report.split(|&c| c == b' ');
            levels
                .next()
                .zip(levels.next())
                .map_or(false, |(first_level, second_level)| {
                    let first = slice_to_usize(first_level);
                    let second = slice_to_usize(second_level);
                    if is_safe_decreasing(first, second) {
                        is_remainder_safe(second, levels, is_safe_decreasing)
                    } else if is_safe_increasing(first, second) {
                        is_remainder_safe(second, levels, is_safe_increasing)
                    } else {
                        false
                    }
                })
        })
        .count()
}

pub fn b(input: &[u8]) -> usize {
    fn dampener_is_safe(
        levels: &[usize],
        position: usize,
        increasing: Option<bool>,
        error_position: Option<usize>,
    ) -> bool {
        if position == levels.len() {
            return true;
        }
        let previous_position = match error_position {
            None => position - 1,
            Some(error) => {
                if error == position - 1 {
                    position - 2
                } else {
                    position - 1
                }
            }
        };
        let current = levels[position];
        let previous = levels[previous_position];
        match increasing {
            None => {
                if is_safe_increasing(previous, current) {
                    if dampener_is_safe(levels, position + 1, Some(true), error_position) {
                        true
                    } else if error_position.is_some() {
                        false
                    } else if dampener_is_safe(levels, position + 1, None, Some(position - 1)) {
                        true
                    } else if dampener_is_safe(levels, position + 1, None, Some(position)) {
                        true
                    } else {
                        false
                    }
                } else if is_safe_decreasing(previous, current) {
                    if dampener_is_safe(levels, position + 1, Some(false), error_position) {
                        true
                    } else if error_position.is_some() {
                        false
                    } else if dampener_is_safe(levels, position + 1, None, Some(position - 1)) {
                        true
                    } else if dampener_is_safe(levels, position + 1, None, Some(position)) {
                        true
                    } else {
                        false
                    }
                } else if error_position.is_some() {
                    false
                } else if dampener_is_safe(levels, position + 1, None, Some(position - 1)) {
                    true
                } else if dampener_is_safe(levels, position + 1, None, Some(position)) {
                    true
                } else {
                    false
                }
            }
            Some(true) => {
                if is_safe_increasing(previous, current) {
                    dampener_is_safe(levels, position + 1, Some(true), error_position)
                } else if error_position.is_some() {
                    false
                } else if dampener_is_safe(levels, position, Some(true), Some(position - 1)) {
                    true
                } else if dampener_is_safe(levels, position + 1, Some(true), Some(position)) {
                    true
                } else {
                    false
                }
            }
            Some(false) => {
                if is_safe_decreasing(previous, current) {
                    dampener_is_safe(levels, position + 1, Some(false), error_position)
                } else if error_position.is_some() {
                    false
                } else if dampener_is_safe(levels, position, Some(false), Some(position - 1)) {
                    true
                } else if dampener_is_safe(levels, position + 1, Some(false), Some(position)) {
                    true
                } else {
                    false
                }
            }
        }
    }
    input
        .split(|&c| c == b'\n')
        .filter(|report| {
            if report.len() < 2 {
                return false;
            }
            let levels: Vec<_> = report.split(|&c| c == b' ').map(slice_to_usize).collect();
            dampener_is_safe(&levels, 1, None, None)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day2a() {
        // given ...
        let input_str = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 2);
    }

    #[test]
    fn test_day2b() {
        // given ...
        let input_str = vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 4);
    }
}
