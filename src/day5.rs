use crate::common::slice_to_usize;
use std::collections::HashMap;

enum Ordering {
    Before(usize),
    After(usize),
}

fn parse_input(input: &[u8]) -> (HashMap<usize, Vec<Ordering>>, Vec<Vec<usize>>) {
    let separator = input.windows(2).position(|w| w == b"\n\n").unwrap();
    let ordering_section = &input[..separator];
    let update_section = &input[separator + 2..];
    let mut ordering = HashMap::new();
    for line in ordering_section.split(|&c| c == b'\n') {
        let mut parts = line.split(|&c| c == b'|');
        let a = slice_to_usize(parts.next().unwrap());
        let b = slice_to_usize(parts.next().unwrap());
        ordering
            .entry(a)
            .or_insert_with(Vec::new)
            .push(Ordering::Before(b));
        ordering
            .entry(b)
            .or_insert_with(Vec::new)
            .push(Ordering::After(a));
    }
    let updates = update_section
        .split(|&c| c == b'\n')
        .map(|line| {
            line.split(|&c| c == b',')
                .map(slice_to_usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (ordering, updates)
}

fn is_update_sequence_correct(ordering: &HashMap<usize, Vec<Ordering>>, updates: &[usize]) -> bool {
    for (position, &update) in updates.iter().enumerate() {
        let rules = ordering.get(&update).unwrap();
        for preceding_position in 0..position {
            let preceding = updates[preceding_position];
            let invalid_preceding = rules
                .iter()
                .any(|rule| matches!(rule, Ordering::Before(n) if *n == preceding));
            if invalid_preceding {
                return false;
            }
            let invalid_succeeding = ordering
                .get(&preceding)
                .unwrap()
                .iter()
                .any(|rule| matches!(rule, Ordering::After(n) if *n == update));
            if invalid_succeeding {
                return false;
            }
        }
    }
    true
}

fn fix_incorrect_update_sequence(
    ordering: &HashMap<usize, Vec<Ordering>>,
    updates: &[usize],
) -> Vec<usize> {
    let mut corrected = updates.to_vec();
    for anchor_position in 0..corrected.len() {
        'anchor: loop {
            let current = corrected[anchor_position];
            let rules = ordering.get(&current).unwrap();
            for scan_position in 0..corrected.len() {
                if scan_position == anchor_position {
                    continue;
                }
                let scan = corrected[scan_position];
                if scan_position < anchor_position {
                    let maybe_invalid_preceding = rules.iter().find_map(|rule| match rule {
                        Ordering::After(n) if *n == scan => Some(scan_position),
                        _ => None,
                    });
                    if let Some(invalid_preceding) = maybe_invalid_preceding {
                        corrected.swap(anchor_position, invalid_preceding);
                        continue 'anchor;
                    }
                } else {
                    let maybe_invalid_succeeding = rules.iter().find_map(|rule| match rule {
                        Ordering::Before(n) if *n == scan => Some(scan_position),
                        _ => None,
                    });
                    if let Some(invalid_succeeding) = maybe_invalid_succeeding {
                        corrected.swap(anchor_position, invalid_succeeding);
                        continue 'anchor;
                    }
                }
            }
            break 'anchor;
        }
    }
    corrected
}

pub fn a(input: &[u8]) -> usize {
    let (ordering, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| is_update_sequence_correct(&ordering, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

pub fn b(input: &[u8]) -> usize {
    let (ordering, updates) = parse_input(input);
    updates
        .iter()
        .filter(|update| !is_update_sequence_correct(&ordering, update))
        .map(|update| fix_incorrect_update_sequence(&ordering, update))
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day5a() {
        // given ...
        let input_str = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 143);
    }

    #[test]
    fn test_day5b() {
        // given ...
        let input_str = vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 123);
    }
}
