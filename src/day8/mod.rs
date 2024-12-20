use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

pub fn day8_part1(content: &String) {
    //let part1 = Instant::now();

    let mut antinode_map: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    // Collect antennas and their positions
    for (y, row) in antinode_map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                let point = Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                antennas.entry(*c).or_insert_with(Vec::new).push(point);
            }
        }
    }

    // For each frequency, calculate antinodes
    for (frequency, points) in antennas.iter() {
        let mut len = points.len();
        for i in 0..len {
            for j in i + 1..len {
                let mut antinodes: Vec<Point> = Vec::new();
                let p1 = points[i];
                let p2 = points[j];

                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;

                let bottom_point: Point;
                let top_point: Point;

                if p2.y > p1.y {
                    bottom_point = p2;
                    top_point = p1;
                } else {
                    bottom_point = p1;
                    top_point = p2;
                }

                let a1 = Point {
                    x: top_point.x + dx,
                    y: top_point.y + dy,
                };

                let a2 = Point {
                    x: bottom_point.x - dx,
                    y: bottom_point.y - dy,
                };

                antinodes.append(&mut vec![a1, a2]);
                add_antinodes(&mut antinode_map, &antinodes)
            }
        }
    }

    let mut antinode_count = 0;
    for row in antinode_map.iter_mut() {
        antinode_count += row.iter().filter(|c| **c == '#').count();
    }

    //print!("Part 1: {}, ", antinode_count);
    //println!("Time elapsed: {:?}", part1.elapsed());
}

pub fn day8_part2(content: &String) {
    //let part2 = Instant::now();

    let mut antinode_map_resonance: Vec<Vec<char>> = content
        .trim()
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    // Collect antennas and their positions
    for (y, row) in antinode_map_resonance.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c != '.' {
                let point = Point {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap(),
                };
                antennas.entry(*c).or_insert_with(Vec::new).push(point);
            }
        }
    }

    // For each frequency, calculate antinodes resonance
    for (frequency, points) in antennas.iter() {
        let mut len = points.len();
        for i in 0..len {
            for j in i + 1..len {
                let mut antinodes: Vec<Point> = Vec::new();
                let p1 = points[i];
                let p2 = points[j];

                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;

                let bottom_point: Point;
                let top_point: Point;

                if p2.y > p1.y {
                    bottom_point = p2;
                    top_point = p1;
                } else {
                    bottom_point = p1;
                    top_point = p2;
                }

                let mut a1 = top_point.clone();
                let mut a2 = bottom_point.clone();

                while within_bounds(&antinode_map_resonance, a1) {
                    a1.x += dx;
                    a1.y += dy;
                    antinodes.push(a1);
                }

                while within_bounds(&antinode_map_resonance, a2) {
                    a2.x -= dx;
                    a2.y -= dy;
                    antinodes.push(a2);
                }

                add_antinodes(&mut antinode_map_resonance, &antinodes)
            }
        }
    }

    let mut antinode_resonance_count = 0;
    for row in antinode_map_resonance.iter_mut() {
        antinode_resonance_count += row.iter().filter(|c| **c != '.').count();
    }

    //print!("Part 2: {}, ", antinode_resonance_count);
    //println!("Time elapsed: {:?}", part2.elapsed());
}

fn within_bounds(grid: &Vec<Vec<char>>, point: Point) -> bool {
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    !(point.x >= cols || point.y >= rows || point.x < 0 || point.y < 0)
}

fn add_antinodes(grid_char: &mut Vec<Vec<char>>, antinodes: &Vec<Point>) {
    for antinode in antinodes.iter() {
        if within_bounds(&grid_char, *antinode) {
            if grid_char[antinode.y as usize][antinode.x as usize] != '#' {
                grid_char[antinode.y as usize][antinode.x as usize] = '#';
            }
        }
    }
}
