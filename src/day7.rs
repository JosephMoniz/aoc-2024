use crate::common::slice_to_usize;

fn concat(a: usize, b: &[u8]) -> usize {
    let b_number = slice_to_usize(b);
    a * (10_usize.pow(b.len() as u32)) + b_number
}

pub fn parse_input(input: &[u8]) -> Vec<(usize, Vec<&[u8]>)> {
    let mut lines = input.split(|&c| c == b'\n');
    let mut equations = Vec::new();
    while let Some(line) = lines.next() {
        let eq_position = line.iter().position(|&c| c == b':').unwrap();
        let result = slice_to_usize(&line[..eq_position]);
        let operands = line[eq_position + 2..].split(|&c| c == b' ').collect();
        equations.push((result, operands));
    }
    equations
}

pub fn a(input: &[u8]) -> usize {
    fn is_valid(result: usize, accumulator: usize, operands: &[&[u8]]) -> bool {
        if operands.is_empty() {
            if result == accumulator {
                return true;
            } else {
                return false;
            }
        }
        if accumulator > result {
            return false;
        }
        let operand = slice_to_usize(operands[0]);
        is_valid(result, accumulator + operand, &operands[1..])
            || is_valid(result, accumulator * operand, &operands[1..])
    }
    parse_input(input)
        .iter()
        .filter(|(result, operands)| is_valid(*result, slice_to_usize(operands[0]), &operands[1..]))
        .map(|(result, _)| result)
        .sum()
}

pub fn b(input: &[u8]) -> usize {
    fn is_valid(result: usize, accumulator: usize, operands: &[&[u8]]) -> bool {
        if operands.is_empty() {
            return if result == accumulator { true } else { false };
        }
        if accumulator > result {
            return false;
        }
        let operand = slice_to_usize(operands[0]);
        is_valid(result, accumulator + operand, &operands[1..])
            || is_valid(result, accumulator * operand, &operands[1..])
            || is_valid(result, concat(accumulator, operands[0]), &operands[1..])
    }
    parse_input(input)
        .iter()
        .filter(|(result, operands)| is_valid(*result, slice_to_usize(operands[0]), &operands[1..]))
        .map(|(result, _)| result)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7a() {
        // given ...
        let input_str = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_day7b() {
        // given ...
        let input_str = vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 11387);
    }
}
