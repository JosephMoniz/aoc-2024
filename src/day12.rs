fn make_empty_regional_map<T: Clone>(fill: T, width: usize, height: usize) -> Vec<Vec<T>> {
    vec![vec![fill; width]; height]
}

fn map_region(
    map: &[&[u8]],
    current: u8,
    x: isize,
    y: isize,
    visited: &mut Vec<Vec<bool>>,
    region: &mut Vec<Vec<bool>>,
) {
    if x < 0 || y < 0 || y >= map.len() as isize || x >= map[y as usize].len() as isize {
        return;
    }
    if visited[y as usize][x as usize] {
        return;
    }
    if map[y as usize][x as usize] != current {
        return;
    }
    visited[y as usize][x as usize] = true;
    region[y as usize][x as usize] = true;
    map_region(map, current, x, y - 1, visited, region);
    map_region(map, current, x + 1, y, visited, region);
    map_region(map, current, x, y + 1, visited, region);
    map_region(map, current, x - 1, y, visited, region);
}

fn calculate_region_area(region: &[Vec<bool>]) -> usize {
    region
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum()
}

fn calculate_region_perimeter(region: &[Vec<bool>]) -> usize {
    let mut perimeter = 0;
    for y in 0..region.len() {
        for x in 0..region[y].len() {
            if !region[y][x] {
                continue;
            }
            if x == 0 || !region[y][x - 1] {
                perimeter += 1;
            }
            if x == region[y].len() - 1 || !region[y][x + 1] {
                perimeter += 1;
            }
            if y == 0 || !region[y - 1][x] {
                perimeter += 1;
            }
            if y == region.len() - 1 || !region[y + 1][x] {
                perimeter += 1;
            }
        }
    }
    perimeter
}

#[derive(Debug, Clone, Copy)]
enum Side {
    Left(usize),
    Right(usize),
    Top(usize),
    Bottom(usize),
}

fn calculate_region_sides(region: &[Vec<bool>]) -> usize {
    // allocate a padded map, looking up coords in this map that corresponds to the original map
    // will have a 1:1 mapping, but the padded map will have a 1 cell border around the original map
    // so lookups will have to be offset by 1 in both x and y
    let mut side_map =
        make_empty_regional_map(Vec::<Side>::new(), region[0].len() + 2, region.len() + 2);
    let mut sides = 0;
    for y in 0..region.len() {
        for x in 0..region[y].len() {
            if !region[y][x] {
                continue;
            }
            if x == 0 || !region[y][x - 1] {
                let above = side_map[y][x].iter().find_map(|side| match side {
                    Side::Left(identified) => Some(*identified),
                    _ => None,
                });
                if let Some(identified) = above {
                    side_map[y + 1][x].push(Side::Left(identified));
                } else {
                    let below = side_map[y + 2][x].iter().find_map(|side| match side {
                        Side::Left(identified) => Some(*identified),
                        _ => None,
                    });
                    if let Some(identified) = below {
                        side_map[y + 1][x].push(Side::Left(identified));
                    } else {
                        sides += 1;
                        side_map[y + 1][x].push(Side::Left(sides));
                    }
                }
            }
            if x == region[y].len() - 1 || !region[y][x + 1] {
                let above = side_map[y][x + 2].iter().find_map(|side| match side {
                    Side::Right(identified) => Some(*identified),
                    _ => None,
                });
                if let Some(identified) = above {
                    side_map[y + 1][x + 2].push(Side::Right(identified));
                } else {
                    let below = side_map[y + 2][x + 2].iter().find_map(|side| match side {
                        Side::Right(identified) => Some(*identified),
                        _ => None,
                    });
                    if let Some(identified) = below {
                        side_map[y + 1][x + 2].push(Side::Right(identified));
                    } else {
                        sides += 1;
                        side_map[y + 1][x + 2].push(Side::Right(sides));
                    }
                }
            }
            if y == 0 || !region[y - 1][x] {
                let left = side_map[y][x].iter().find_map(|side| match side {
                    Side::Top(identified) => Some(*identified),
                    _ => None,
                });
                if let Some(identified) = left {
                    side_map[y][x + 1].push(Side::Top(identified));
                } else {
                    let right = side_map[y][x + 2].iter().find_map(|side| match side {
                        Side::Top(identified) => Some(*identified),
                        _ => None,
                    });
                    if let Some(identified) = right {
                        side_map[y][x + 1].push(Side::Top(identified));
                    } else {
                        sides += 1;
                        side_map[y][x + 1].push(Side::Top(sides));
                    }
                }
            }
            if y == region.len() - 1 || !region[y + 1][x] {
                let left = side_map[y + 2][x].iter().find_map(|side| match side {
                    Side::Bottom(identified) => Some(*identified),
                    _ => None,
                });
                if let Some(identified) = left {
                    side_map[y + 2][x + 1].push(Side::Bottom(identified));
                } else {
                    let right = side_map[y + 2][x + 2].iter().find_map(|side| match side {
                        Side::Bottom(identified) => Some(*identified),
                        _ => None,
                    });
                    if let Some(identified) = right {
                        side_map[y + 2][x + 1].push(Side::Bottom(identified));
                    } else {
                        sides += 1;
                        side_map[y + 2][x + 1].push(Side::Bottom(sides));
                    }
                }
            }
        }
    }
    sides
}

pub fn a(input: &[u8]) -> usize {
    let map = input.split(|&c| c == b'\n').collect::<Vec<_>>();
    let mut visited = make_empty_regional_map(false, map[0].len(), map.len());
    let mut fencing_cost = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited[y][x] {
                continue;
            }
            let mut region = make_empty_regional_map(false, map[0].len(), map.len());
            map_region(
                &map,
                map[y][x],
                x as isize,
                y as isize,
                &mut visited,
                &mut region,
            );
            let area = calculate_region_area(&region);
            let perimeter = calculate_region_perimeter(&region);
            fencing_cost += area * perimeter;
        }
    }
    fencing_cost
}

pub fn b(input: &[u8]) -> usize {
    let map = input.split(|&c| c == b'\n').collect::<Vec<_>>();
    let mut visited = make_empty_regional_map(false, map[0].len(), map.len());
    let mut fencing_cost = 0;
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if visited[y][x] {
                continue;
            }
            let mut region = make_empty_regional_map(false, map[0].len(), map.len());
            map_region(
                &map,
                map[y][x],
                x as isize,
                y as isize,
                &mut visited,
                &mut region,
            );
            let area = calculate_region_area(&region);
            let sides = calculate_region_sides(&region);
            fencing_cost += area * sides;
        }
    }
    fencing_cost
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day12a() {
        // given ...
        let input_str = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = a(input);

        // then ...
        assert_eq!(result, 1930);
    }

    #[test]
    fn test_day12b() {
        // given ...
        let input_str = vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
        .join("\n");
        let input = input_str.as_bytes();

        // when ...
        let result = b(input);

        // then ...
        assert_eq!(result, 1206);
    }
}
