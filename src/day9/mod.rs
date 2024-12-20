pub fn day9_part1(content: &String) {
    let disk: Vec<u16> = content
        .trim()
        .chars()
        .map(|ch| ch.to_string().parse::<u16>().unwrap())
        .collect();

    let mut fragmented = fragment(&disk);

    let mut file_index = fragmented.len() - 1;

    for i in 0..fragmented.len() {
        if fragmented[i] == -1 && fragmented[i..].iter().filter(|n| **n != -1).count() != 0 {
            fragmented[i] = fragmented[file_index];
            fragmented[file_index] = -1;
            loop {
                if fragmented[file_index] != -1 {
                    break;
                }

                file_index -= 1;
            }
        }
    }

    let defragmented: Vec<usize> = fragmented
        .iter()
        .filter(|n| **n != -1)
        .map(|n| *n as usize)
        .collect();
    let mut checksum: usize = 0;

    for i in 0..defragmented.len() {
        checksum += i * defragmented[i];
    }

    println!("Part 1: {}", checksum);
}

pub fn day9_part2(content: &String) {
    let disk: Vec<u16> = content
        .trim()
        .chars()
        .map(|ch| ch.to_string().parse::<u16>().unwrap())
        .collect();

    let fragmented = fragment(&disk);
    let compacted = compact_files(fragmented);

    let mut checksum: usize = 0;

    for (i, n) in compacted.iter().enumerate() {
        if *n != -1 {
            checksum += *n as usize * i;
        }
    }

    println!("Part 2: {}", checksum);
}

fn fragment(disk: &Vec<u16>) -> Vec<isize> {
    let mut id = 0;
    let mut fragmented = Vec::<isize>::new();

    for (index, c) in disk.iter().enumerate() {
        if index % 2 == 0 {
            for i in 0..*c as i32 {
                fragmented.push(id);
            }

            id += 1;
        } else {
            for i in 0..*c as i32 {
                fragmented.push(-1);
            }
        }
    }

    fragmented
}

fn compact_files(mut disk_blocks: Vec<isize>) -> Vec<isize> {
    let mut moved: Vec<isize> = Vec::new();
    let mut file_ids = Vec::new();
    let mut start_index = 0;

    while start_index < disk_blocks.len() {
        if disk_blocks[start_index] != -1 {
            let file_id = disk_blocks[start_index];
            let file_len = disk_blocks
                .iter()
                .skip(start_index)
                .take_while(|&&c| c == file_id)
                .count();
            file_ids.push((file_id, start_index, file_len));
            start_index += file_len;
        } else {
            start_index += 1;
        }
    }

    for &(file_id, start_index, length) in file_ids.iter().rev() {
        let mut free_space_start = 0;
        let mut free_space_length = 0;

        while free_space_start < disk_blocks.len() {
            if disk_blocks[free_space_start] == -1 {
                let current_free_length = disk_blocks
                    .iter()
                    .skip(free_space_start)
                    .take_while(|&&c| c == -1)
                    .count();
                if current_free_length >= length {
                    free_space_length = current_free_length;
                    break;
                }
                free_space_start += current_free_length;
            } else {
                free_space_start += 1;
            }
        }

        if free_space_length >= length {
            let file_block = &disk_blocks[start_index..start_index + length];

            let mut new_disk_blocks = vec![-1; disk_blocks.len()];
            new_disk_blocks.splice(
                free_space_start..free_space_start + length,
                file_block.to_vec(),
            );
            moved.append(&mut file_block.to_vec());

            for (i, &ch) in disk_blocks.iter().enumerate() {
                if i < free_space_start || i >= free_space_start + length {
                    new_disk_blocks[i] = ch;
                }
            }

            // remove duplicates from last
            moved.sort();

            for i in (0..new_disk_blocks.len()).rev() {
                if moved.contains(&(new_disk_blocks[i])) {
                    new_disk_blocks[i] = -1;
                    moved.pop();
                }
            }

            disk_blocks = new_disk_blocks;
            //println!("New disk blocks: {:?}", disk_blocks.iter().map(|x| x.to_string()).collect::<String>().replace("-1", "."));
        }
    }

    disk_blocks
}
