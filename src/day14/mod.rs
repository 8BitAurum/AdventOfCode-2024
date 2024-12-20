const C: usize = 101;
const R: usize = 103;

pub fn day14_part1(content: &str) {
    let mut room = [[0; C]; R];

    let rows = room.len();
    let cols = room[0].len();

    let mut safety_score = 1;

    for bot_info in content.trim().lines().collect::<Vec<&str>>() {
        let info = bot_info.split(' ').collect::<Vec<&str>>();

        let pos = info[0].split('=').collect::<Vec<&str>>()[1];
        let vel = info[1].split('=').collect::<Vec<&str>>()[1];

        let (x, y): (i32, i32) = (
            pos.split(",").collect::<Vec<_>>()[0].parse().unwrap(),
            pos.split(',').collect::<Vec<_>>()[1].parse().unwrap(),
        );

        let (vx, vy): (i32, i32) = (
            vel.split(",").collect::<Vec<_>>()[0].parse().unwrap(),
            vel.split(',').collect::<Vec<_>>()[1].parse().unwrap(),
        );

        let mut nx = (x + (100 * vx)).abs() % cols as i32;
        let mut ny = (y + (100 * vy)).abs() % rows as i32;

        if vx < 0 {
            nx = cols as i32 - nx;
        }
        if vy < 0 {
            ny = rows as i32 - ny;
        }

        if nx == cols as i32 {
            nx = 0;
        }
        if ny == rows as i32 {
            ny = 0;
        }

        room[ny as usize][nx as usize] += 1;
    }

    //display_grid(&room);
    //println!("==================================================================================");

    //top left
    let quad_1: i32 = room
        .iter()
        .take(rows / 2)
        .map(|s| s.iter().take(cols / 2).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| **x)
        .sum();

    // top right
    let quad_2: i32 = room
        .iter()
        .take(rows / 2) //taking the rest, not 11, as there are less than 11 elements
        .map(|s| {
            s.iter()
                .skip(cols / 2 + 1)
                .take(10000000000)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| **x)
        .sum();

    // bottom right
    let quad_3: i32 = room
        .iter()
        .skip(rows / 2 + 1)
        .take(10000000000)
        .map(|s| {
            s.iter()
                .skip(cols / 2 + 1)
                .take(10000000000)
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| **x)
        .sum();

    // bottom left
    let quad_4: i32 = room
        .iter()
        .skip(rows / 2 + 1)
        .take(10000000000)
        .map(|s| s.iter().take(cols / 2).collect::<Vec<_>>())
        .flatten()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| **x)
        .sum();

    safety_score *= quad_1 * quad_2 * quad_3 * quad_4;
    println!("Part 1: {}", safety_score);
}

pub fn day14_part2(content: &str) {
    let mut room = [[0; C]; R];
    let mut positions: Vec<(i32, i32)> = Vec::new();
    let mut velocities: Vec<(i32, i32)> = Vec::new();

    for bot_info in content.trim().lines().collect::<Vec<&str>>() {
        let info = bot_info.split(' ').collect::<Vec<&str>>();

        let pos = info[0].split('=').nth(1).unwrap();
        let vel = info[1].split('=').nth(1).unwrap();

        positions.push((
            pos.split(",").nth(0).unwrap().parse().unwrap(),
            pos.split(',').nth(1).unwrap().parse().unwrap(),
        ));

        velocities.push((
            vel.split(",").nth(0).unwrap().parse().unwrap(),
            vel.split(',').nth(1).unwrap().parse().unwrap(),
        ));
    }

    // populate the grid with the current position
    populate_grid(&mut room, &positions, &Vec::new());

    //===============================================================
    let mut curr_positions = Vec::new();
    let mut prev_positions = positions;

    let mut time_to_tree = 1;

    while true {
        curr_positions = move_all_bots(&prev_positions, &velocities);
        populate_grid(&mut room, &curr_positions, &prev_positions);
        prev_positions = curr_positions;

        let mut not_overlap = true;

        for row in &room {
            if row.iter().max().unwrap() > &1 {
                not_overlap = false;
            }
        }

        if not_overlap {
            break;
        }
        time_to_tree += 1;
    }
    //===============================================================

    //display_grid(&room);

    println!("Part 2: {}", time_to_tree);
}

fn populate_grid(
    room: &mut [[i32; C]; R],
    new_positions: &Vec<(i32, i32)>,
    old_positions: &Vec<(i32, i32)>,
) {
    for position in new_positions {
        let mut ny = position.1 as usize;
        let mut nx = position.0 as usize;
        room[ny][nx] += 1;
    }

    for position in old_positions {
        let mut oy = position.1 as usize;
        let mut ox = position.0 as usize;
        room[oy][ox] -= 1;
    }
}

fn display_grid(grid: &[[i32; C]; R]) {
    for row in grid {
        for x in row {
            if *x != 0 {
                print!("{}", x)
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn move_bot(position: &(i32, i32), velocity: &(i32, i32)) -> (i32, i32) {
    let rows = R as i32;
    let cols = C as i32;

    let mut nx = (position.0 + velocity.0).abs() % cols;
    let mut ny = (position.1 + velocity.1).abs() % rows;

    if position.0 + velocity.0 < 0 {
        nx = cols - nx;
    }
    if position.1 + velocity.1 < 0 {
        ny = rows - ny;
    }

    if nx == cols {
        nx = 0;
    }
    if ny == rows {
        ny = 0;
    }

    (nx, ny)
}

fn move_all_bots(positions: &Vec<(i32, i32)>, velocities: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    for i in 0..positions.len() {
        result.push(move_bot(&positions[i], &velocities[i]))
    }

    result
}
