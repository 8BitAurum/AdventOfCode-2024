use std::collections::{HashSet, VecDeque};

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// TODO: PART 2 not done yet
pub fn day15(content: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut move_cmds: Vec<char> = Vec::new();

    let mut bot_pos: (i32, i32) = (0, 0);
    let mut l_box_coords: Vec<(i32, i32)> = Vec::new();

    let mut gps_coords = 0;

    for line in content.trim().lines() {
        if line.contains("#") {
            let mut row = vec![];
            for char in line.trim().chars() {
                if char == '#' {
                    row.push('#');
                    row.push('#');
                }

                if char == '.' {
                    row.push('.');
                    row.push('.');
                }

                if char == 'O' {
                    row.push('[');
                    row.push(']');
                }

                if char == '@' {
                    row.push('@');
                    row.push('.');
                }
            }

            grid.push(row);
        }

        if line.contains("^") || line.contains("v") || line.contains("<") || line.contains(">") {
            move_cmds.append(&mut line.chars().collect());
        }
    }

    // filter the move commands to remove newlines (including carriage return for windows)
    move_cmds.iter().filter(|&&c| c != '\n');
    move_cmds.iter().filter(|&&c| c != '\r');

    let rows = grid.len();
    let cols = grid[0].len();

    // find starting position of the bot
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '@' {
                bot_pos = (x as i32, y as i32);
            }

            if grid[y][x] == '[' {
                l_box_coords.push((x as i32, y as i32));
            }
        }
    }

    println!("============================INITIAL STATE==========================");
    display_grid(&grid);
    println!("================================================================");

    for cmd in move_cmds {
        let update = update_positions(&grid, &bot_pos, cmd);
        let (dx, dy) = update.1;

        let points_to_change = update.0;

        for point in points_to_change {
            // remove the old point from box coords and add the new one
            if l_box_coords.contains(&point) {
                if let Some(pos) = l_box_coords.iter().position(|x| *x == point) {
                    l_box_coords.remove(pos);
                }
                l_box_coords.push((point.0 + dx, point.1 + dy));
            }
        }

        bot_pos.0 += dx;
        bot_pos.1 += dy;

        println!(
            "============================COMMAND {}==========================",
            cmd
        );
        update_grid(&mut grid, &bot_pos, &l_box_coords);
        display_grid(&grid);
        println!("================================================================");
    }

    for coords in l_box_coords {
        gps_coords += 100 * coords.1 + coords.0;
    }

    println!("Part 2: {}", gps_coords);
}

fn update_positions(
    grid: &Vec<Vec<char>>,
    bot_pos: &(i32, i32),
    cmd: char,
) -> (Vec<(i32, i32)>, (i32, i32)) {
    let mut direction = (0, 0);
    let mut points_to_change: Vec<(i32, i32)> = vec![];

    let mut queue = VecDeque::new();
    queue.push_back(*bot_pos);

    match cmd {
        '^' => {
            direction = DIRECTIONS[2];
        }
        '>' => {
            direction = DIRECTIONS[1];
        }
        'v' => {
            direction = DIRECTIONS[0];
        }
        '<' => {
            direction = DIRECTIONS[3];
        }
        _ => {}
    }

    let (dx, dy) = direction;
    let mut movement_blocked = false;

    while !queue.is_empty() && !movement_blocked {
        let mut current = queue.pop_front().unwrap();

        loop {
            let check_point = (current.0 + dx, current.1 + dy);

            match grid[check_point.1 as usize][check_point.0 as usize] {
                '#' => {
                    direction = (0, 0);
                    return (points_to_change, check_point);
                }
                '[' => {
                    points_to_change.push(check_point);
                }
                ']' => {
                    points_to_change.push((check_point.0 - 1, check_point.1));
                    queue.push_back((check_point.0 - 1, check_point.1));
                }
                '.' => break,
                _ => {}
            }
            current = check_point;
        }

        /*
        let mut wall_check: HashSet<(i32, i32)> = HashSet::new();

        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == '[' || grid[y][x] == ']' {
                    wall_check.insert((x as i32, y as i32));
                }
            }
        }


        for point in &wall_check {
            if check_for_wall(grid, *point, direction) {
                direction = (0, 0);
                movement_blocked = true;
            }
        }
         */
    }
    (points_to_change, direction)
}

fn display_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn update_grid(grid: &mut Vec<Vec<char>>, bot_pos: &(i32, i32), l_box_coords: &Vec<(i32, i32)>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '#' {
                continue;
            }

            if l_box_coords.contains(&(x as i32, y as i32)) {
                grid[y][x] = '[';
            } else if (x as i32, y as i32) == *bot_pos {
                grid[y][x] = '@';
            } else {
                grid[y][x] = '.'
            }
        }
    }

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '#' {
                continue;
            }

            if grid[y][x - 1] == '[' {
                grid[y][x] = ']';
            }
        }
    }
}

fn check_for_wall(grid: &Vec<Vec<char>>, position: (i32, i32), direction: (i32, i32)) -> bool {
    grid[(position.1 + direction.1) as usize][(position.0 + direction.0) as usize] == '#'
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

/*
pub fn day15(content: &str) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut move_cmds: Vec<char> = Vec::new();

    let mut bot_pos: (i32, i32) = (0, 0);
    let mut box_coords: Vec<(i32, i32)> = Vec::new();

    let mut gps_coords = 0;

    for line in content.trim().lines() {
        if line.contains("#") {
            grid.push(line.chars().collect());
        }

        if line.contains("^") || line.contains("v") || line.contains("<") || line.contains(">") {
            move_cmds.append(&mut line.chars().collect());
        }
    }

    // filter the move commands to remove newlines (including carriage return for windows)
    move_cmds.iter().filter(|&&c| c != '\n');
    move_cmds.iter().filter(|&&c| c != '\r');

    let rows = grid.len();
    let cols = grid[0].len();

    // find starting position of the bot
    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '@' {
                bot_pos = (x as i32, y as i32);
            }

            if grid[y][x] == 'O' {
                box_coords.push((x as i32, y as i32));
            }
        }
    }

    for cmd in move_cmds {
        let update = update_positions(&grid, &bot_pos, cmd);
        let (dx, dy) = update.1;

        let points_to_change = update.0;

        for point in points_to_change {
            // remove the old point from box coords and add the new one
            if box_coords.contains(&point) {
                if let Some(pos) = box_coords.iter().position(|x| *x == point) {
                    box_coords.remove(pos);
                }
                box_coords.push((point.0 + dx, point.1 + dy));
            } else if point == bot_pos {
                bot_pos = (point.0 + dx, point.1 + dy);
            }
        }

        update_grid(&mut grid, &bot_pos, &box_coords);
    }

    for coords in box_coords {
        gps_coords += 100*coords.1 + coords.0;
    }

    println!("Part 1: {}", gps_coords);
}

fn update_positions(grid: &Vec<Vec<char>>, bot_pos: &(i32, i32), cmd: char) -> (Vec<(i32, i32)>, (i32, i32)){
    let mut direction = (0, 0);
    let mut points_to_change: Vec<(i32, i32)> = vec![*bot_pos];
    let mut current: (i32, i32) = *bot_pos;

    match cmd {
        '^' => { direction = DIRECTIONS[2]; },
        '>' => { direction = DIRECTIONS[1]; },
        'v' => { direction = DIRECTIONS[0]; },
        '<' => { direction = DIRECTIONS[3]; },
        _ => {}
    }

    let (dx, dy) = direction;

    loop {
        let check_point = (current.0 + dx, current.1 + dy);

        match grid[check_point.1 as usize][check_point.0 as usize] {
            '#' => {
                direction = (0, 0);
                break;
            },
            'O' => { points_to_change.push((check_point.0, check_point.1)); },
            '.' => break,
            _ => {}
        }
        current = check_point;
    }
    (points_to_change, direction)
}

fn display_grid(grid: &Vec<Vec<char>>) {
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn update_grid(grid: &mut Vec<Vec<char>>, bot_pos: &(i32, i32), box_coords: &Vec<(i32, i32)>) {
    let rows = grid.len();
    let cols = grid[0].len();

    for y in 0..rows {
        for x in 0..cols {
            if grid[y][x] == '#' { continue; }

            if box_coords.contains(&(x as i32, y as i32)) { grid[y][x] = 'O' }
            else if (x as i32, y as i32) == *bot_pos { grid[y][x] = '@' }
            else { grid[y][x] = '.' }
        }
    }
}
 */
