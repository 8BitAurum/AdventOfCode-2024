use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn day12(content: &str) {
    let mut grid: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let grid_clone = grid.clone();
    let mut cost = 0;
    let mut cost_discounted = 0;

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != '.' {
                let current = grid[i][j].clone();
                let mut result = get_area(
                    &mut grid,
                    current,
                    Point {
                        x: i as i32,
                        y: j as i32,
                    },
                );
                let mut info = Vec::new();

                for point in &result.1 {
                    info.push(find_neighbour_count(current, *point, &grid_clone));
                }

                let mut perimeter = 0;
                for (_point, neighbours) in &info {
                    perimeter += 4 - *neighbours;
                }

                cost += result.0 * perimeter as usize; //Part 1

                // number of corners and sides are the same in a shape, so
                // count the corners instead
                cost_discounted += result.0 * find_corners(&result.1);

                grid[i][j] = '.';
            }
        }
    }

    println!("Part 1: {}", cost);
    println!("Part 2: {}", cost_discounted);
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

fn get_area(grid: &mut Vec<Vec<char>>, plant: char, start: Point) -> (usize, Vec<Point>) {
    let mut area = 1;
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    let mut points: Vec<Point> = Vec::new();

    while let Some(point) = queue.pop_front() {
        let mut neighbours = 0;

        for &(dx, dy) in &DIRECTIONS {
            let nx = point.x as i32 + dx;
            let ny = point.y as i32 + dy;

            if nx >= 0 && ny >= 0 && nx < grid.len() as i32 && ny < grid[0].len() as i32 {
                let nxu = nx as usize;
                let nyu = ny as usize;

                let point = Point { x: nx, y: ny };

                if grid[nxu][nyu] == plant && !visited.contains(&point) {
                    neighbours += 1;
                    area += 1;
                    queue.push_back(point);
                    visited.insert(point);
                    grid[nxu][nyu] = '.';
                }
            }
        }

        points.push(point);
    }

    points.sort_by(|a, b| a.x.cmp(&b.x));
    (area, points)
}

fn find_corners(points: &Vec<Point>) -> usize {
    let mut corners = 0;

    for point in points {
        //outer corners
        corners += (!points.contains(&&Point {
            x: point.x - 1,
            y: point.y,
        }) && !points.contains(&&Point {
            x: point.x,
            y: point.y - 1,
        })) as usize;

        corners += (!points.contains(&&Point {
            x: point.x + 1,
            y: point.y,
        }) && !points.contains(&&Point {
            x: point.x,
            y: point.y - 1,
        })) as usize;

        corners += (!points.contains(&&Point {
            x: point.x - 1,
            y: point.y,
        }) && !points.contains(&&Point {
            x: point.x,
            y: point.y + 1,
        })) as usize;

        corners += (!points.contains(&&Point {
            x: point.x + 1,
            y: point.y,
        }) && !points.contains(&&Point {
            x: point.x,
            y: point.y + 1,
        })) as usize;

        // inner corners

        corners += (points.contains(&&Point {
            x: point.x - 1,
            y: point.y,
        }) && points.contains(&&Point {
            x: point.x,
            y: point.y - 1,
        }) && !points.contains(&&Point {
            x: point.x - 1,
            y: point.y - 1,
        })) as usize;

        corners += (points.contains(&&Point {
            x: point.x + 1,
            y: point.y,
        }) && points.contains(&&Point {
            x: point.x,
            y: point.y - 1,
        }) && !points.contains(&&Point {
            x: point.x + 1,
            y: point.y - 1,
        })) as usize;

        corners += (points.contains(&&Point {
            x: point.x - 1,
            y: point.y,
        }) && points.contains(&&Point {
            x: point.x,
            y: point.y + 1,
        }) && !points.contains(&&Point {
            x: point.x - 1,
            y: point.y + 1,
        })) as usize;

        corners += (points.contains(&&Point {
            x: point.x + 1,
            y: point.y,
        }) && points.contains(&&Point {
            x: point.x,
            y: point.y + 1,
        }) && !points.contains(&&Point {
            x: point.x + 1,
            y: point.y + 1,
        })) as usize;
    }

    corners
}

fn find_neighbour_count(plant: char, point: Point, grid: &Vec<Vec<char>>) -> (Point, u8) {
    let mut neighbours: u8 = 0;

    for (nx, ny) in get_neighbors(
        point.x as usize,
        point.y as usize,
        grid.len(),
        grid[0].len(),
    ) {
        if grid[nx][ny] == plant {
            neighbours += 1;
        }
    }

    (point, neighbours)
}
