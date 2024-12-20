use std::fs::File;
use std::io::Read;

pub fn day7_part1() {
    let mut content = String::new();
    File::open("./src/day7/day7_input_test.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<Vec<usize>> = Vec::new();
    let mut checked: usize = 0;

    for line in content.trim().lines() {
        let split: Vec<&str> = line.split(':').collect();
        lhs.push(split[0].parse::<usize>().unwrap());

        rhs.push(
            split[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
    }

    for i in 0..rhs.len() {
        if try_solution_part1(&lhs[i], &rhs[i]) {
            checked += lhs[i];
        }
    }

    println!("Part 1: {}", checked);
}

pub fn day7_part2() {
    let mut content = String::new();
    File::open("./src/day7/day7_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<Vec<usize>> = Vec::new();
    let mut checked: usize = 0;

    for line in content.trim().lines() {
        let split: Vec<&str> = line.split(':').collect();
        lhs.push(split[0].parse::<usize>().unwrap());

        rhs.push(
            split[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
    }

    for i in 0..rhs.len() {
        if try_solution_part2(&lhs[i], &rhs[i]) {
            checked += lhs[i];
        }
    }

    println!("Part 2: {}", checked);
}

fn try_solution_part1(lhs: &usize, rhs: &[usize]) -> bool {
    let n = rhs.len();
    let iterations = 2_i32.pow((n - 1) as u32);
    let mut permutation = vec![-1; n - 1];
    for i in 0..iterations {
        for j in 0..n - 1 {
            if i % 2_i32.pow(j as u32) == 0 {
                permutation[j] *= -1;
            }
        }

        let mut sum = rhs[0];

        for i in 1..rhs.len() {
            if permutation[i - 1] == -1 {
                sum += rhs[i];
            } else if permutation[i - 1] == 1 {
                sum *= rhs[i];
            }
        }

        if sum == *lhs {
            return true;
        }
    }
    false
}

fn try_solution_part2(lhs: &usize, rhs: &[usize]) -> bool {
    let n = rhs.len();
    let iterations = 3_i32.pow((n - 1) as u32);
    let mut permutation = vec![-1; n - 1];

    for i in 0..iterations {
        for j in 0..n - 1 {
            if i % 3_i32.pow(j as u32) == 0 {
                match permutation[j] {
                    0 => permutation[j] = 1,
                    1 => permutation[j] = -1,
                    -1 => permutation[j] = 0,
                    _ => continue,
                }
            }
        }

        let mut sum = rhs[0];

        for i in 1..rhs.len() {
            if permutation[i - 1] == -1 {
                sum += rhs[i]
            } else if permutation[i - 1] == 1 {
                sum *= rhs[i]
            } else if permutation[i - 1] == 0 {
                let mut result = String::new();
                result.push_str(&sum.to_string());
                result.push_str(&rhs[i].to_string());

                sum = result.parse().unwrap();
            }
        }

        if sum == *lhs {
            return true;
        }
    }

    false
}

pub fn day7_part1_optimized() {
    let mut content = String::new();
    File::open("./src/day7/day7_input_test.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut lhs: Vec<usize> = Vec::new();
    let mut rhs: Vec<Vec<usize>> = Vec::new();
    let mut checked: usize = 0;

    for line in content.trim().lines() {
        let split: Vec<&str> = line.split(':').collect();
        lhs.push(split[0].parse::<usize>().unwrap());

        rhs.push(
            split[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect(),
        );
    }

    //TODO: implement optimized

    println!("Part 1 (optimized): {}", checked);
}
