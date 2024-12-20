use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

pub fn day20_part1(content: &str) {
    let mut grid: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut start: Point = Point::new(0, 0);
    let mut end: Point = Point::new(0, 0);

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'S' {
                start = Point::new(x, y);
                grid[y][x] = '.';
            }

            if grid[y][x] == 'E' {
                end = Point::new(x, y);
                grid[y][x] = '.';
            }
        }
    }

    let normal_path_length = if let Some(min_steps) = find_path(&grid, start, end) {
        min_steps
    } else {
        0
    };

    let mut working_cheat: Vec<usize> = Vec::new();

    for y in 1..rows {
        for x in 1..cols {
            if grid[y][x] == '#' {
                let mut cheat_map = grid.clone();
                cheat_map[y][x] = '.';

                let cheat_length = if let Some(steps) = find_path(&cheat_map, start, end) {
                    steps
                } else {
                    0
                };

                if cheat_length < normal_path_length {
                    working_cheat.push(cheat_length);
                }
            }
        }
    }

    println!("{}", working_cheat.iter().filter(|&&x| normal_path_length - x >= 100).count());
}

// TODO: NOT DONE YET
pub fn day20_part2(content: &str) {
    let mut grid: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut start: Point = Point::new(0, 0);
    let mut end: Point = Point::new(0, 0);

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == 'S' {
                start = Point::new(x, y);
                grid[y][x] = '.';
            }

            if grid[y][x] == 'E' {
                end = Point::new(x, y);
                grid[y][x] = '.';
            }
        }
    }

    let normal_path_length = if let Some(min_steps) = find_path(&grid, start, end) {
        min_steps
    } else {
        0
    };

    let mut working_cheat: Vec<usize> = Vec::new();

    for y in 1..rows {
        for x in 1..cols {
            if grid[y][x] == '#' {
                let mut cheat_map = grid.clone();
                cheat_map[y][x] = '.';

                let mut cheat_time = 1;

                nuke_circle(&mut cheat_map, Point::new(x, y));

                let cheat_length = if let Some(steps) = find_path(&cheat_map, start, end) {
                    steps
                } else {
                    0
                };

                if cheat_length < normal_path_length {
                    working_cheat.push(cheat_length);
                }
            }
        }
    }

    //println!("{}", working_cheat.iter().filter(|&&x| normal_path_length - x >= 100).count());
    println!("{}", working_cheat.len());
}

fn nuke_circle(map: &mut Vec<Vec<char>>, center: Point) {
    let rows = map.len();
    let cols = map[0].len();

    let radius = 10;

    let min_y = if center.y as i32 - radius/2 < 0 { 0 } else { center.y - (radius/2) as usize };
    let min_x = if center.x as i32 - radius/2 < 0 { 0 } else { center.x - (radius/2) as usize };

    let max_y = if center.y + (radius/2) as usize > rows { rows } else { center.y + (radius/2) as usize };
    let max_x = if center.x + (radius/2) as usize > cols { cols } else { center.x + (radius/2) as usize };

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let dx = x.abs_diff(center.x);
            let dy = y.abs_diff(center.y);

            if dx*dx + dy*dy <= 100 && x < cols && y < rows && x > 0 && y > 0 {
                map[y][x] = '.';
            }
        }
    }
}

fn find_path(grid: &Vec<Vec<char>>, start: Point, end: Point) -> Option<usize> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut queue = VecDeque::new();
    queue.push_back((start, 0));

    let mut visited = vec![vec![false; cols]; rows];
    visited[start.y][start.x] = true;

    while let Some((point, dist)) = queue.pop_front() {
        if point == end {
            return Some(dist);
        }

        for &(dx, dy) in &DIRECTIONS {
            let new_x = point.x as i32 + dx;
            let new_y = point.y as i32 + dy;

            if new_x >= 0 && new_x < cols as i32 && new_y >= 0 && new_y < rows as i32 {
                let new_point = Point::new(new_x as usize, new_y as usize);

                if grid[new_point.y][new_point.x] == '.' && !visited[new_point.y][new_point.x] {
                    visited[new_point.y][new_point.x] = true;
                    queue.push_back((new_point, dist + 1));
                }
            }
        }
    }

    None // If no path is found, return None
}

fn get_neighbors(x: usize, y: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
    DIRECTIONS
        .iter()
        .filter_map(|&(dx, dy)| {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                Some((new_x as usize, new_y as usize))
            } else {
                None
            }
        })
        .collect()
}
