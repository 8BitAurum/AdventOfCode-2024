extern crate core;

use crate::day1::day1;
use crate::day10::day10;
use crate::day11::day11;
use crate::day12::day12;
use crate::day13::day13;
use crate::day14::{day14_part1, day14_part2};
use crate::day15::day15;
use crate::day16::day16;
use crate::day17::day17_part1;
use crate::day18::day18;
use crate::day19::day19;
use crate::day2::day2;
use crate::day4::{day4_part1, day4_part2};
use crate::day5::day5;
use crate::day6::day6;
use crate::day7::{day7_part1, day7_part1_optimized, day7_part2};
use crate::day8::{day8_part1, day8_part2};
use crate::day9::{day9_part1, day9_part2};
use std::fs::File;
use std::io::Read;
use std::ops::Div;
use std::time::{Duration, Instant};
use crate::day20::{day20_part1, day20_part2};

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod test;

fn main() {
    println!("Advent of Code 2024");

    /*
    println!("==================Day 1===================");
    let one = Instant::now();
    day1();
    println!("Total run time: {:?}", one.elapsed());

    println!("==================Day 2===================");
    let two = Instant::now();
    day2();
    println!("Total run time: {:?}", two.elapsed());

    println!("==================Day 3===================");
    let three = Instant::now();
    test3();
    println!("Total run time: {:?}", three.elapsed());

    println!("==================Day 4===================");
    let four = Instant::now();
    day4_part1();
    day4_part2();
    println!("Total run time: {:?}", four.elapsed());

    println!("==================Day 5===================");
    let five = Instant::now();
    day5();
    println!("Total run time: {:#?}", five.elapsed());

     */

    /*
    println!("==================Day 6===================");
    let six = Instant::now();
    day6();
    println!("Total run time: {:#?}", six.elapsed());
     */

    /*
    println!("==================Day 7===================");
    let seven = Instant::now();
    day7_part1_optimized();
    //day7_part2();
    println!("Total run time: {:#?}", seven.elapsed());

     */

    /*

    println!("==================Day 8===================");
    let mut content = String::new();
    File::open("./src/day8/day8_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day8_part1(&content);
    day8_part2(&content);
    println!("Done in {:?}", time.elapsed());

     */

    /*
    println!("==================Day 9===================");
    let mut content = String::new();
    File::open("./src/day9/day9_input.txt").unwrap().read_to_string(&mut content).unwrap();
    //day9_part1(&content);
    day9_part2(&content);

     */

    /*
    println!("==================Day 10===================");
    let mut content = String::new();
    File::open("./src/day10/day10_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day10(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 11===================");
    let mut content = String::new();
    File::open("./src/day11/day11_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day11(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 12===================");
    let mut content = String::new();
    File::open("./src/day12/day12_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day12(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 13===================");
    let mut content = String::new();
    File::open("./src/day13/day13_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day13(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 14===================");
    let mut content = String::new();
    File::open("./src/day14/day14_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    //day14_part1(&content);
    day14_part2(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 15===================");
    let mut content = String::new();
    File::open("./src/day15/day15_input_test.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day15(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 16===================");
    let mut content = String::new();
    File::open("./src/day16/day16_input_test.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day16(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 17===================");
    let mut content = String::new();
    File::open("./src/day17/day17_input_test.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day17_part1(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 18===================");
    let mut content = String::new();
    File::open("./src/day18/day18_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day18(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    /*
    println!("==================Day 19===================");
    let mut content = String::new();
    File::open("./src/day19/day19_input.txt").unwrap().read_to_string(&mut content).unwrap();
    let time = Instant::now();
    day19(&content);
    println!("Time taken : {:?}", time.elapsed());
     */

    println!("==================Day 20===================");
    let mut content = String::new();
    File::open("./src/day20/day20_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let time = Instant::now();
    //day20_part1(&content);
    day20_part2(&content);
    println!("Time taken : {:?}", time.elapsed());
}
