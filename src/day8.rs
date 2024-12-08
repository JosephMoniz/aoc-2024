use std::collections::HashMap;

pub fn parse_input(input: &[u8]) -> (Vec<Vec<u8>>, HashMap<u8, Vec<(usize, usize)>>) {
    let map: Vec<Vec<_>> = input
        .split(|&c| c == b'\n')
        .map(|row| row.iter().cloned().collect())
        .collect();
    let mut freq_coords: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();
    for (y, row) in map.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell != b'.' {
                freq_coords
                    .entry(cell)
                    .and_modify(|coords| coords.push((x, y)))
                    .or_insert(vec![(x, y)]);
            }
        }
    }
    (map, freq_coords)
}

pub fn signed_coords(x: usize, y: usize) -> (isize, isize) {
    (x as isize, y as isize)
}

pub fn get_dimensional_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> (isize, isize) {
    let (x1i, y1i) = signed_coords(x1, y1);
    let (x2i, y2i) = signed_coords(x2, y2);
    (x1i - x2i, y1i - y2i)
}

pub fn apply_dimensional_distance(x: usize, y: usize, dx: isize, dy: isize) -> (isize, isize) {
    let (xi, yi) = signed_coords(x, y);
    (xi + dx, yi + dy)
}

pub fn apply_inverted_dimensional_distance(
    x: usize,
    y: usize,
    dx: isize,
    dy: isize,
) -> (isize, isize) {
    let (xi, yi) = signed_coords(x, y);
    (xi - dx, yi - dy)
}

pub fn is_new_anti_node(map: &[Vec<u8>], x: isize, y: isize) -> bool {
    if coords_are_on_map(map, x, y) && map[y as usize][x as usize] != b'#' {
        true
    } else {
        false
    }
}

pub fn coords_are_on_map(map: &[Vec<u8>], x: isize, y: isize) -> bool {
    x >= 0 && x < map[0].len() as isize && y >= 0 && y < map.len() as isize
}

pub fn a(input: &[u8]) -> usize {
    let (mut map, freq_coords) = parse_input(input);
    let mut total_anti_nodes = 0;
    for (_, coords) in freq_coords.iter() {
        for &(x1, y1) in coords {
            let mut antinodes = 0;
            for &(x2, y2) in coords {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                let (dx, dy) = get_dimensional_distance(x1, y1, x2, y2);
                let (x1i, y1i) = apply_dimensional_distance(x1, y1, dx, dy);
                if is_new_anti_node(&map, x1i, y1i) {
                    map[y1i as usize][x1i as usize] = b'#';
                    antinodes += 1;
                }
                let (x2i, y2i) = apply_inverted_dimensional_distance(x2, y2, dx, dy);
                if is_new_anti_node(&map, x2i, y2i) {
                    map[y2i as usize][x2i as usize] = b'#';
                    antinodes += 1;
                }
            }
            total_anti_nodes += antinodes;
        }
    }
    total_anti_nodes
}

pub fn b(input: &[u8]) -> usize {
    let (mut map, freq_coords) = parse_input(input);
    let mut total_anti_nodes = 0;
    for (_, coords) in freq_coords.iter() {
        for &(x1, y1) in coords {
            let mut antinodes = 0;
            for &(x2, y2) in coords {
                if x1 == x2 && y1 == y2 {
                    continue;
                }
                if map[y1][x1] != b'#' {
                    map[y1][x1] = b'#';
                    antinodes += 1;
                }
                if map[y2][x2] != b'#' {
                    map[y2][x2] = b'#';
                    antinodes += 1;
                }
                let (dx, dy) = get_dimensional_distance(x1, y1, x2, y2);
                let (mut x1i, mut y1i) = signed_coords(x1, y1);
                loop {
                    (x1i, y1i) = apply_dimensional_distance(x1i as usize, y1i as usize, dx, dy);
                    if !coords_are_on_map(&map, x1i, y1i) {
                        break;
                    }
                    if is_new_anti_node(&map, x1i, y1i) {
                        map[y1i as usize][x1i as usize] = b'#';
                        antinodes += 1;
                    }
                }
                let (mut x2i, mut y2i) = signed_coords(x2, y2);
                loop {
                    (x2i, y2i) =
                        apply_inverted_dimensional_distance(x2i as usize, y2i as usize, dx, dy);
                    if !coords_are_on_map(&map, x2i, y2i) {
                        break;
                    }
                    if is_new_anti_node(&map, x2i, y2i) {
                        map[y2i as usize][x2i as usize] = b'#';
                        antinodes += 1;
                    }
                }
            }
            total_anti_nodes += antinodes;
        }
    }
    total_anti_nodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8a() {
        // given ...
        let input_str = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 14);
    }

    #[test]
    fn test_day8b() {
        // given ...
        let input_str = vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 34);
    }
}
