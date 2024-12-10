fn calculate_trail_head_score(
    map: &[&[u8]],
    x: isize,
    y: isize,
    expected: u8,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    if x < 0 || y < 0 || y >= map.len() as isize || x >= map[y as usize].len() as isize {
        return 0;
    }
    let current = map[y as usize][x as usize];
    if current != expected {
        return 0;
    }
    if current == b'9' && !visited[y as usize][x as usize] {
        visited[y as usize][x as usize] = true;
        return 1;
    }
    // north
    let north = calculate_trail_head_score(map, x, y - 1, expected + 1, visited);
    // east
    let east = calculate_trail_head_score(map, x + 1, y, expected + 1, visited);
    // south
    let south = calculate_trail_head_score(map, x, y + 1, expected + 1, visited);
    // west
    let west = calculate_trail_head_score(map, x - 1, y, expected + 1, visited);
    north + east + south + west
}

fn calculate_trail_head_rating(map: &[&[u8]], x: isize, y: isize, expected: u8) -> usize {
    if x < 0 || y < 0 || y >= map.len() as isize || x >= map[y as usize].len() as isize {
        return 0;
    }
    let current = map[y as usize][x as usize];
    if current != expected {
        return 0;
    }
    if current == b'9' {
        return 1;
    }
    // north
    let north = calculate_trail_head_rating(map, x, y - 1, expected + 1);
    // east
    let east = calculate_trail_head_rating(map, x + 1, y, expected + 1);
    // south
    let south = calculate_trail_head_rating(map, x, y + 1, expected + 1);
    // west
    let west = calculate_trail_head_rating(map, x - 1, y, expected + 1);
    north + east + south + west
}

pub fn a(input: &[u8]) -> usize {
    let map_buffer = input.split(|&c| c == b'\n').collect::<Vec<_>>();
    let map = &map_buffer;
    (0..map.len())
        .flat_map(|y| {
            (0..map[y].len()).map(move |x| {
                let mut visited = vec![vec![false; map[y].len()]; map.len()];
                let score =
                    calculate_trail_head_score(map, x as isize, y as isize, b'0', &mut visited);
                score
            })
        })
        .sum()
}

pub fn b(input: &[u8]) -> usize {
    let map_buffer = input.split(|&c| c == b'\n').collect::<Vec<_>>();
    let map = &map_buffer;
    (0..map.len())
        .flat_map(|y| {
            (0..map[y].len()).map(move |x| {
                let score = calculate_trail_head_rating(map, x as isize, y as isize, b'0');
                score
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10a() {
        // given ...
        let input_str = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 36);
    }

    #[test]
    fn test_day10b() {
        // given ...
        let input_str = vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 81);
    }
}
