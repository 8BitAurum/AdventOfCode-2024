use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};

pub fn day6() {
    let mut content = String::new();
    File::open("./src/day6/day6_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let grid: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let rows = grid.len();
    let cols = grid[0].len();

    let start_direction: (i32, i32) = find_start_info(&grid).1;
    let start_pos = find_start_info(&grid).0;

    let unique_points: HashSet<(usize, usize)> =
        HashSet::from_iter(find_path(&grid, start_pos, start_direction).0);
    println!("Part 1: {}", unique_points.len());

    //=============================================================================================
    //=========================== PART 2 ==========================================================
    //=============================================================================================

    let mut loop_obstacles = 0;

    for i in 0..rows {
        for j in 0..cols {
            let mut check = grid.clone();
            if check[i][j] == '.' {
                check[i][j] = '#';
                if find_path(&check, find_start_info(&check).0, find_start_info(&check).1).1 {
                    loop_obstacles += 1;
                }
            }
        }
    }

    println!("Part 2: {}", loop_obstacles);
}

//=================================================================================================
//=============================== HELPER FUNCTIONS ================================================
//=================================================================================================

fn find_start_info(grid: &Vec<Vec<char>>) -> ((usize, usize), (i32, i32)) {
    let mut start_pos: (usize, usize) = (0, 0);
    let mut start_direction = (0, 0);

    let rows = grid.len();
    let cols = grid[0].len();

    let directions = vec![
        (1, 0),  //right
        (0, 1),  //down
        (-1, 0), //left
        (0, -1), //up
    ];

    //find out the starting position and direction to headed in
    //this is a generalized version, whereas in the puzzle input, guard always
    //starts facing up "^"
    for i in 0..rows {
        for j in 0..cols {
            match grid[i][j] {
                '^' => {
                    start_direction = directions[3];
                    start_pos = (i, j);
                }

                '>' => {
                    start_direction = directions[0];
                    start_pos = (i, j);
                }
                'v' => {
                    start_direction = directions[1];
                    start_pos = (i, j);
                }
                '<' => {
                    start_direction = directions[2];
                    start_pos = (i, j);
                }
                _ => continue,
            }
        }
    }

    (start_pos, start_direction)
}

fn find_path(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    start_direction: (i32, i32),
) -> (Vec<(usize, usize)>, bool) {
    let time = Instant::now();
    let mut path: Vec<(usize, usize)> = Vec::new();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut looped = false;

    let mut iterations: usize = 0;

    let mut direction = start_direction;

    let mut next_dir = 0;

    let mut left_map = false;

    let directions = vec![
        (1, 0),  //right
        (0, 1),  //down
        (-1, 0), //left
        (0, -1), //up
    ];

    let mut curr_x: i32 = start.1 as i32;
    let mut curr_y: i32 = start.0 as i32;

    while !left_map {
        if next_dir > 3 {
            next_dir = 0
        }

        let mut dx = direction.0;
        let mut dy = direction.1;

        if grid[curr_y as usize][curr_x as usize] != '#' {
            path.push((curr_y as usize, curr_x as usize));
            iterations += 1;
        } else {
            curr_x -= dx;
            curr_y -= dy;

            direction = directions[next_dir];
            next_dir += 1;

            dx = direction.0;
            dy = direction.1;
        }

        curr_x += dx;
        curr_y += dy;

        if curr_y >= rows as i32 || curr_x >= cols as i32 || curr_y < 0 || curr_x < 0 {
            left_map = true;
        }

        // giving a timeout duration so that the function understands that the path
        // is an infinite loop

        // the timeout duration is hardcoded, I haven't found an alternative
        // to finding an infinite path yet
        if time.elapsed() > Duration::from_millis(3) {
            looped = true;
            break;
        }
    }

    (path, looped)

    //TODO: optimize finding looped paths
    //have a path function and keep adding points into a set
    // if one duplicate point is there, make an overlap bool true, and if there is another duplicate
    // point when overlap is true then its a loop
}
