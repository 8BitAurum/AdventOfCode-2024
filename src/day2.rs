use std::fs::File;
use std::io::Read;

pub fn aoc2() {
    let mut content = String::new();
    File::open("./src/day2_input.txt").unwrap().read_to_string(&mut content).unwrap();

    let levels: Vec<_> = content.trim().lines().collect();
    let mut levels_parsed: Vec<Vec<u32>> = Vec::new();

    for level in levels {
        levels_parsed.push(
            level.split_whitespace().into_iter()
                .filter_map(|x| x.parse::<u32>().ok()).collect::<Vec<u32>>()
        );
    }

    let mut safe = 0;
    let mut safe_dampened  = 0;

    for level in &levels_parsed {
        if check_safety(&level) {
            safe += 1; // Part 1
            safe_dampened += 1;
        } else {
            for i in 0..level.len() {
                let mut check = level.clone();
                check.remove(i);                                        // PART 2
                if check_safety(&check) {
                    safe_dampened += 1;
                    break
                }
            }
        }
    }

    println!("Safe: {}", safe);
    println!("Dampened Safe: {}", safe_dampened)
}

fn check_safety(level: &[u32]) -> bool {
    let mut flag = level.is_sorted() || level.iter().is_sorted_by(|a, b| a >= b);
    for i in 0..level.len() - 1 {
        if (level[i + 1].abs_diff(level[i]) > 3) || (level[i + 1].abs_diff(level[i]) < 1) {
            flag = false;
        }
    }

    flag
}