use std::collections::HashMap;

pub fn day11(content: &str) {
    let mut stone_counts: HashMap<usize, usize> = HashMap::new();

    for stone in content.trim().split_whitespace().into_iter() {
        stone_counts
            .entry(stone.parse().unwrap())
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut stone_count_clone = stone_counts.clone();

    println!("Part 1: {}", blink(25, &mut stone_counts));
    println!("Part 2: {}", blink(75, &mut stone_count_clone));
}

fn blink(n: usize, stone_counts: &mut HashMap<usize, usize>) -> usize {
    for _i in 0..n {
        let mut new_stone_counts: HashMap<usize, usize> = HashMap::new();

        for key in stone_counts.keys() {
            if *key == 0 {
                new_stone_counts
                    .entry(1)
                    .and_modify(|counter| *counter += *stone_counts.get(key).unwrap_or(&0))
                    .or_insert(*stone_counts.get(&0).unwrap_or(&0));
            } else if key.to_string().len() % 2 == 0 {
                let len = key.to_string().len();
                let left: &str = &key.to_string()[..(len / 2)];
                let right: &str = &key.to_string()[(len / 2)..];

                new_stone_counts
                    .entry(left.parse().unwrap())
                    .and_modify(|counter| *counter += *stone_counts.get(key).unwrap_or(&0))
                    .or_insert(*stone_counts.get(key).unwrap_or(&0));

                new_stone_counts
                    .entry(right.parse().unwrap())
                    .and_modify(|counter| *counter += *stone_counts.get(key).unwrap_or(&0))
                    .or_insert(*stone_counts.get(key).unwrap_or(&0));
            } else {
                new_stone_counts
                    .entry(*key * 2024)
                    .and_modify(|counter| *counter += *stone_counts.get(key).unwrap_or(&0))
                    .or_insert(*stone_counts.get(key).unwrap_or(&0));
            }
        }

        *stone_counts = new_stone_counts;
    }

    stone_counts.values().into_iter().sum::<usize>()
}
