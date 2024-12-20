use std::collections::HashMap;
use std::collections::HashSet;

fn can_form_design(
    design: &str,
    towels: &HashSet<String>,
    memo: &mut HashMap<String, bool>,
) -> bool {
    if design.is_empty() {
        return true;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }

    for i in 1..=design.len() {
        let prefix = &design[0..i];
        if towels.contains(prefix) {
            if can_form_design(&design[i..], towels, memo) {
                memo.insert(design.to_string(), true);
                return true;
            }
        }
    }

    memo.insert(design.to_string(), false);
    false
}

fn count_ways(design: &str, towels: &HashSet<String>, memo: &mut HashMap<String, u64>) -> u64 {
    if design.is_empty() {
        return 1;
    }
    if let Some(&result) = memo.get(design) {
        return result;
    }

    let mut ways = 0;
    for i in 1..=design.len() {
        let prefix = &design[0..i];
        if towels.contains(prefix) {
            ways += count_ways(&design[i..], towels, memo);
        }
    }

    memo.insert(design.to_string(), ways);
    ways
}

pub fn day19(content: &str) {
    let towel_patterns = content
        .trim()
        .lines()
        .nth(0)
        .unwrap()
        .split(',')
        .map(|x| x.trim())
        .collect::<Vec<&str>>();

    let mut designs = Vec::new();

    for line in content.trim().lines() {
        if !line.contains(',') && !line.is_empty() {
            designs.push(line.trim());
        }
    }

    let towel_set: HashSet<String> = towel_patterns.into_iter().map(String::from).collect();
    let mut memo = HashMap::new();
    let mut possible_designs_count = 0;

    for design in &designs {
        if can_form_design(*design, &towel_set, &mut memo) {
            possible_designs_count += 1;
        }
    }

    let mut memo = HashMap::new();
    let mut ways = 0;
    for design in &designs {
        ways += count_ways(*design, &towel_set, &mut memo);
    }

    println!("Part 1: {}", possible_designs_count);
    println!("Part 2: {}", ways);
}
