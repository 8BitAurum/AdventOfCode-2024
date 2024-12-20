use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

pub fn day10(content: &String) {
    let grid: Vec<Vec<i32>> = content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let mut total_score = 0;
    let mut total_rating = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                total_score += find_reachable(&grid, (i, j));
                total_rating += get_distinct_paths(i, j, &grid);
            }
        }
    }

    println!("Part 1: {}", total_score);
    println!("Part 2: {}", total_rating);
}

// Breadth First Search Algorithm
fn find_reachable(map: &Vec<Vec<i32>>, start: (usize, usize)) -> usize {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // start with the head of path
    queue.push_back(start);
    visited.insert(start);

    let mut reachable_nines = 0;

    while let Some((x, y)) = queue.pop_front() {
        // end of path reached, hence this path is valid and reachable
        if map[x][y] == 9 {
            reachable_nines += 1;
        }

        //we search in every adjacent node for a possible path node
        for &(dx, dy) in &DIRECTIONS {
            //find neighbours
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            // check if neighbours are within bounds
            if nx >= 0 && ny >= 0 && nx < map.len() as i32 && ny < map[0].len() as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                // if the path is valid, push that path point onto the queue
                // and mark that point as visited

                // if end point is marked as visited, other paths that lead to
                // same end point is not taken into account, removing duplicates
                if map[nx][ny] == map[x][y] + 1 && !visited.contains(&(nx, ny)) {
                    queue.push_back((nx, ny));
                    visited.insert((nx, ny));
                }
            }
        }
    }

    reachable_nines
}

// code to get all the neighbouring nodes of a node
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

// This breadth first search algorithm uses a queue of paths
// as opposed to the queue of visited points in the first implementation
// to get every distinct path
fn get_distinct_paths(x: usize, y: usize, map: &Vec<Vec<i32>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut distinct_paths = 0;

    queue.push_back(vec![(x, y)]);

    while let Some(path) = queue.pop_front() {
        // revisiting the last node in a path
        let (cx, cy) = *path.last().unwrap();

        // reached the end of the path (node is terminal), so it's a distinct path
        if map[cx][cy] == 9 {
            distinct_paths += 1;
            continue;
        }

        // Explore neighbors of the node, since the path hasn't ended yet
        for (nx, ny) in get_neighbors(cx, cy, rows, cols) {
            // check if the point hasn't been visited, and is also a valid path node
            if !visited.contains(&(nx, ny)) && map[nx][ny] == map[cx][cy] + 1 {
                // create a branching path with neighbour and add it into the queue
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                queue.push_back(new_path);
            }
        }

        // Mark current position as visited in the set
        visited.insert((cx, cy));
    }

    distinct_paths
}
