use std::ops::Deref;

fn omni_directional_linear_matches(grid: &[&[u8]], x: usize, y: usize, target: &[u8]) -> usize {
    if target.is_empty() || grid[y][x] != target[0] {
        return 0;
    }
    let remaining = &target[1..];
    let mut count = 0;
    if linear_north_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_north_east_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_east_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_south_east_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_south_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_south_west_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_west_match(grid, x, y, remaining) {
        count += 1;
    }
    if linear_north_west_match(grid, x, y, remaining) {
        count += 1;
    }
    count
}

fn linear_north_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let x = x;
    let mut y = y;
    for c in remaining {
        if y == 0 {
            return false;
        }
        y -= 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_north_east_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let mut y = y;
    for c in remaining {
        if y == 0 || x == grid[y].len() - 1 {
            return false;
        }
        y -= 1;
        x += 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_east_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let y = y;
    for c in remaining {
        if x == grid[y].len() - 1 {
            return false;
        }
        x += 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_south_east_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let mut y = y;
    for c in remaining {
        if y == grid.len() - 1 || x == grid[y].len() - 1 {
            return false;
        }
        y += 1;
        x += 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_south_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let x = x;
    let mut y = y;
    for c in remaining {
        if y == grid.len() - 1 {
            return false;
        }
        y += 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_south_west_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let mut y = y;
    for c in remaining {
        if y == grid.len() - 1 || x == 0 {
            return false;
        }
        y += 1;
        x -= 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_west_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let y = y;
    for c in remaining {
        if x == 0 {
            return false;
        }
        x -= 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

fn linear_north_west_match(grid: &[&[u8]], x: usize, y: usize, remaining: &[u8]) -> bool {
    let mut x = x;
    let mut y = y;
    for c in remaining {
        if y == 0 || x == 0 {
            return false;
        }
        y -= 1;
        x -= 1;
        if grid[y][x] != *c {
            return false;
        }
    }
    true
}

pub fn a(input: &[u8]) -> usize {
    let grid_buffer: Vec<_> = input.split(|&c| c == b'\n').collect();
    let grid = grid_buffer.deref();
    (0..grid.len())
        .flat_map(|y| {
            (0..grid[y].len()).map(move |x| omni_directional_linear_matches(grid, x, y, b"XMAS"))
        })
        .sum()
}

pub fn b(input: &[u8]) -> usize {
    let grid_buffer: Vec<_> = input.split(|&c| c == b'\n').collect();
    let grid = grid_buffer.deref();
    (0..grid.len())
        .flat_map(|y| {
            (0..grid[y].len()).filter(move |x| {
                // center
                if grid[y][*x] != b'A' {
                    return false;
                }

                if y == 0 || *x == 0 || y == grid.len() - 1 || *x == grid[y].len() - 1 {
                    return false;
                }

                // north-west to south-east
                let north_west = grid[y - 1][*x - 1];
                let south_east = grid[y + 1][*x + 1];
                if north_west == b'M' {
                    if south_east != b'S' {
                        return false;
                    }
                } else if north_west == b'S' {
                    if south_east != b'M' {
                        return false;
                    }
                } else {
                    return false;
                }

                // north-east to south-west
                let north_east = grid[y - 1][*x + 1];
                let south_west = grid[y + 1][*x - 1];
                if north_east == b'M' {
                    if south_west != b'S' {
                        return false;
                    }
                } else if north_east == b'S' {
                    if south_west != b'M' {
                        return false;
                    }
                } else {
                    return false;
                }

                true
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day4a() {
        // given ...
        let input_str = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 18);
    }

    #[test]
    fn test_day4b() {
        // given ...
        let input_str = vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 9);
    }
}
