use std::fs::File;
use std::io::Read;

pub fn aoc1() {
    let mut content = String::new();
    File::open("./src/day1_input.txt").unwrap().read_to_string(&mut content).unwrap();

    let mut lines: Vec<_> = content.trim().split("\r\n").collect();

    let mut left:Vec<u32> = Vec::new();
    let mut right:Vec<u32> = Vec::new();
    let mut similarity_score: Vec<_> = Vec::new();

    for line in lines.iter_mut() {
        let split: Vec<&str> = line.split("   ").collect();

        left.push(split[0].parse::<u32>().unwrap());
        right.push(split[1].parse::<u32>().unwrap());
    }

    left.sort();
    right.sort();

    let mut sum:u32 = 0;

    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]); //PART 1
        similarity_score.push(left[i] * count_freq(&right, left[i])); //PART 2
    }

    println!("Day 1, first part: {}", sum);
    println!("Day 1, second part: {}", similarity_score.iter().sum::<u32>());
}

fn count_freq(list: &[u32], e: u32) -> u32 {
    let mut count = 0;
    for i in list {
        if *i == e {
            count += 1;
        }
    }

    count
}