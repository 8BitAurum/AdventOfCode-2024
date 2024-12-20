use std::fs::File;
use std::io::Read;

pub fn day4_part1() {
    let mut content = String::new();
    File::open("./src/day4/day4_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in content.trim().lines() {
        grid.push(line.chars().collect());
    }

    // slopes/directions to find XMAS in
    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
        (-1, 1),  // up-right
    ];

    let mut result = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;
    let word = "XMAS";
    let word_len = word.len();

    for i in 0..rows {
        for j in 0..cols {
            for &(dx, dy) in &directions {
                let mut x = i;
                let mut y = j;
                let mut match_found = true;

                for k in 0..word_len {
                    if x < 0
                        || x >= rows
                        || y < 0
                        || y >= cols
                        || grid[x as usize][y as usize] != word.chars().nth(k).unwrap()
                    {
                        match_found = false;
                        break;
                    }
                    x += dx;
                    y += dy;
                }

                if match_found {
                    result += 1;
                }
            }
        }
    }

    println!("Part 1: {}", result);
}

pub fn day4_part2() {
    let mut content = String::new();
    File::open("./src/day4/day4_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in content.trim().lines() {
        grid.push(line.chars().collect());
    }

    let mut result = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            let mut bounded = true;

            if grid[i][j] == 'A' {
                if i - 1 < 0 || j - 1 < 0 || i + 1 > rows || j + 1 > cols {
                    bounded = false;
                }

                if bounded {
                    if grid[i - 1][j - 1] == 'M'
                        && grid[i + 1][j + 1] == 'S'
                        && grid[i + 1][j - 1] == 'M'
                        && grid[i - 1][j + 1] == 'S'
                    {
                        result += 1
                    }

                    if grid[i - 1][j - 1] == 'S'
                        && grid[i + 1][j + 1] == 'M'
                        && grid[i + 1][j - 1] == 'S'
                        && grid[i - 1][j + 1] == 'M'
                    {
                        result += 1
                    }

                    if grid[i - 1][j - 1] == 'M'
                        && grid[i + 1][j + 1] == 'S'
                        && grid[i + 1][j - 1] == 'S'
                        && grid[i - 1][j + 1] == 'M'
                    {
                        result += 1
                    }

                    if grid[i - 1][j - 1] == 'S'
                        && grid[i + 1][j + 1] == 'M'
                        && grid[i + 1][j - 1] == 'M'
                        && grid[i - 1][j + 1] == 'S'
                    {
                        result += 1
                    }
                }
            }
        }
    }

    println!("Part 2: {}", result);
}
