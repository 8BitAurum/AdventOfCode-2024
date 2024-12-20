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

pub fn day18(content: &str) {
    let lines = content.trim().lines().collect::<Vec<&str>>();
    let grid_size = 71;
    let mut grid = vec![vec![true; grid_size]; grid_size]; // true = safe, false = corrupted
                                                           // The list of incoming byte positions (from the puzzle input)
    let mut byte_positions = vec![];

    for i in 0..1024 {
        let parsed = lines[i].split(',').collect::<Vec<&str>>();
        let x = parsed[0].parse::<usize>().unwrap();
        let y = parsed[1].parse::<usize>().unwrap();

        byte_positions.push((x, y));
    }

    for &(x, y) in &byte_positions {
        if x < grid_size && y < grid_size {
            grid[y][x] = false; // False means corrupted byte
        }
    }

    // Define the start and end points (top-left and bottom-right corners)
    let start = Point::new(0, 0);
    let end = Point::new(grid_size - 1, grid_size - 1);

    if let Some(min_steps) = find_path(&grid, start, end) {
        println!("Part 1: {}", min_steps);
    } else {
        println!("There is no path to the exit.");
    }

    // we start from 1025 since we already know the first 1024 bytes do not block
    // path to the exit
    for i in 1025..lines.len() {
        let parsed = lines[i].split(',').collect::<Vec<&str>>();
        let x = parsed[0].parse::<usize>().unwrap();
        let y = parsed[1].parse::<usize>().unwrap();

        // directly mark the part on grid as false
        grid[y][x] = false;

        if let Some(_min_steps) = find_path(&grid, start, end) {
            continue;
        } else {
            println!("Part 2: {:?}", (x, y));
            break;
        }
    }
}

fn find_path(grid: &Vec<Vec<bool>>, start: Point, end: Point) -> Option<usize> {
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

                if grid[new_point.y][new_point.x] && !visited[new_point.y][new_point.x] {
                    visited[new_point.y][new_point.x] = true;
                    queue.push_back((new_point, dist + 1));
                }
            }
        }
    }

    None // If no path is found, return None
}
