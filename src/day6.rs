use std::ops::Deref;

#[derive(Copy, Clone, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

pub fn find_starting_position(grid: &[&[u8]]) -> Option<(usize, usize)> {
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == b'^' {
                return Some((x, y));
            }
        }
    }
    None
}

pub fn trace_original_path(grid: &[&[u8]]) -> (usize, Vec<Vec<bool>>) {
    let (mut x, mut y) = find_starting_position(grid.deref()).unwrap();
    let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
    let mut direction = Direction::North;
    let mut unique_coords = 0;
    loop {
        if !visited[y][x] {
            visited[y][x] = true;
            unique_coords += 1;
        }
        match direction {
            Direction::North => {
                if y == 0 {
                    break;
                }
                if grid[y - 1][x] != b'#' {
                    y -= 1;
                } else {
                    direction = Direction::East;
                }
            }
            Direction::East => {
                if x == grid[0].len() - 1 {
                    break;
                }
                if grid[y][x + 1] != b'#' {
                    x += 1;
                } else {
                    direction = Direction::South;
                }
            }
            Direction::South => {
                if y == grid.len() - 1 {
                    break;
                }
                if grid[y + 1][x] != b'#' {
                    y += 1;
                } else {
                    direction = Direction::West;
                }
            }
            Direction::West => {
                if x == 0 {
                    break;
                }
                if grid[y][x - 1] != b'#' {
                    x -= 1;
                } else {
                    direction = Direction::North;
                }
            }
        }
    }
    (unique_coords, visited)
}

pub fn a(input: &[u8]) -> usize {
    let grid = input
        .split(|&c| c == b'\n')
        .map(|row| row)
        .collect::<Vec<_>>();
    let (unique_coords, _) = trace_original_path(grid.deref());
    unique_coords
}

pub fn b(input: &[u8]) -> usize {
    let grid = input
        .split(|&c| c == b'\n')
        .map(|row| row)
        .collect::<Vec<_>>();
    let (_, original_visited) = trace_original_path(grid.deref());
    let original_coords = original_visited
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, &visited)| visited)
                .map(move |(x, _)| (x, y))
        })
        .collect::<Vec<_>>();
    let (init_x, init_y) = find_starting_position(grid.deref()).unwrap();
    let mut unique_loops = 0;
    for (blocked_x, blocked_y) in original_coords {
        let mut x = init_x;
        let mut y = init_y;
        let mut visited = vec![vec![vec![]; grid[0].len()]; grid.len()];
        let mut direction = Direction::North;
        'loop_check: loop {
            if !visited[y][x].contains(&direction) {
                visited[y][x].push(direction);
            } else {
                unique_loops += 1;
                break 'loop_check;
            }
            match direction {
                Direction::North => {
                    if y == 0 {
                        break;
                    }
                    if grid[y - 1][x] != b'#' && (x != blocked_x || y - 1 != blocked_y) {
                        y -= 1;
                    } else {
                        direction = Direction::East;
                    }
                }
                Direction::East => {
                    if x == grid[0].len() - 1 {
                        break;
                    }
                    if grid[y][x + 1] != b'#' && (x + 1 != blocked_x || y != blocked_y) {
                        x += 1;
                    } else {
                        direction = Direction::South;
                    }
                }
                Direction::South => {
                    if y == grid.len() - 1 {
                        break;
                    }
                    if grid[y + 1][x] != b'#' && (x != blocked_x || y + 1 != blocked_y) {
                        y += 1;
                    } else {
                        direction = Direction::West;
                    }
                }
                Direction::West => {
                    if x == 0 {
                        break;
                    }
                    if grid[y][x - 1] != b'#' && (x - 1 != blocked_x || y != blocked_y) {
                        x -= 1;
                    } else {
                        direction = Direction::North;
                    }
                }
            }
        }
    }
    unique_loops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day6a() {
        // given ...
        let input_str = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 41);
    }

    #[test]
    fn test_day6b() {
        // given ...
        let input_str = vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 6);
    }
}
