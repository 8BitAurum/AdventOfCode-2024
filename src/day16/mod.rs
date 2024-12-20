use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // N, E, S, W


// TODO: PART 2 not done yet
#[derive(Clone, Eq, PartialEq)]
struct State {
    x: usize,
    y: usize,
    direction: usize, // 0 = North, 1 = East, 2 = South, 3 = West
    score: usize,
}

impl State {
    fn new(x: usize, y: usize, direction: usize, score: usize) -> Self {
        State {
            x,
            y,
            direction,
            score,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn day16(content: &str) {
    let grid = content
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let rows = grid.len();
    let cols = grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if grid[x][y] == 'S' {
                start_pos = (x, y);
            }
            if grid[x][y] == 'E' {
                end_pos = (x, y);
            }
        }
    } // Position of 'E'

    let result = find_lowest_score(grid, start_pos, end_pos);
    println!("Part 1: {}", result);
}

fn find_lowest_score(maze: Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let rows = maze.len();
    let cols = maze[0].len();

    let mut heap = BinaryHeap::new();
    let mut visited = HashMap::new();

    let start_state = State::new(start.0, start.1, 1, 0); // Starting at S, facing East
    heap.push(start_state.clone());
    visited.insert((start.0, start.1, 1), 0);

    while let Some(current_state) = heap.pop() {
        let (x, y, dir, score) = (
            current_state.x,
            current_state.y,
            current_state.direction,
            current_state.score,
        );

        // If we reach the end position, return the score
        if (x, y) == end {
            return score;
        }

        // Try to move forward
        let new_x = (x as i32 + DIRECTIONS[dir].0) as usize;
        let new_y = (y as i32 + DIRECTIONS[dir].1) as usize;

        if new_x < rows && new_y < cols && maze[new_x][new_y] != '#' {
            let new_score = score + 1;

            // we push only the states with minimum scores into the heap
            if visited.get(&(new_x, new_y, dir)).unwrap_or(&usize::MAX) > &new_score {
                visited.insert((new_x, new_y, dir), new_score);
                heap.push(State::new(new_x, new_y, dir, new_score));
            }
        }

        // Try rotating clockwise (90 degrees)
        let new_dir = (dir + 1) % 4;
        let new_score = score + 1000;
        if visited.get(&(x, y, new_dir)).unwrap_or(&usize::MAX) > &new_score {
            visited.insert((x, y, new_dir), new_score);
            heap.push(State::new(x, y, new_dir, new_score));
        }

        // Try rotating counterclockwise (90 degrees)
        let new_dir = (dir + 3) % 4;
        let new_score = score + 1000;
        if visited.get(&(x, y, new_dir)).unwrap_or(&usize::MAX) > &new_score {
            visited.insert((x, y, new_dir), new_score);
            heap.push(State::new(x, y, new_dir, new_score));
        }
    }

    usize::MAX // If no valid path is found
}
