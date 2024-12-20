use regex::Regex;
use std::fs::File;
use std::io::Read;

pub fn day3_part_1_regex() {
    let mut content = String::new();
    File::open("./src/day3/day3_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();
    let mut prod_sum = 0;
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let processed = regex.captures_iter(&content);

    for item in processed {
        prod_sum += &item[1].parse::<i32>().unwrap() * &item[2].parse::<i32>().unwrap();
    }
    println!("{}", prod_sum);
}

pub fn day3_parser() {
    //TODO: implement day 3 parser method
}
