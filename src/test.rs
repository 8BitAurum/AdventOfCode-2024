fn check_safety(level: &[u32]) -> bool {
    let mut flag = true;
    for i in 0..level.len() - 1 {
        if (level[i + 1].abs_diff(level[i]) > 3) || (level[i + 1].abs_diff(level[i]) < 1) {
            flag = false;
        }
    }

    flag
}

