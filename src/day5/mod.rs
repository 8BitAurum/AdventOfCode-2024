use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub fn day5() {
    let mut content = String::new();
    File::open("./src/day5/day5_input.txt")
        .unwrap()
        .read_to_string(&mut content)
        .unwrap();

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut updates: Vec<Vec<u32>> = Vec::new();

    let mut update_sum = 0;
    let mut update_sum_modified = 0;

    for line in content.lines() {
        if line.contains('|') {
            let rule = line
                .split('|')
                .into_iter()
                .filter_map(|x| x.parse::<u32>().ok())
                .collect::<Vec<u32>>();

            if !rules.contains_key(&rule[0]) {
                rules.insert(rule[0], vec![rule[1]]);
            } else {
                rules.get_mut(&rule[0]).unwrap().push(rule[1]);
            }
        }

        if line.contains(',') {
            updates.push(
                line.split(',')
                    .into_iter()
                    .filter_map(|x| x.parse::<u32>().ok())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    for update in &mut updates {
        let sorted =
            update.is_sorted_by(|a, b| rules.get(a).is_some() && rules.get(a).unwrap().contains(b));

        if sorted {
            update_sum += update[(update.len() - 1) / 2];
        } else {
            update.sort_by(|a, b| {
                if rules.get(a).is_some() && rules.get(a).unwrap().contains(b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            update_sum_modified += update[update.len() / 2]; //TIL integer division truncation was a thing
        }
    }

    println!("Part 1: {}", update_sum);
    println!("Part 2: {}", update_sum_modified);
}
