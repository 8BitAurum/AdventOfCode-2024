pub fn day13(content: &str) {
    let mut cost: f64 = 0f64;

    for machine in content.split("\r\n\r\n").collect::<Vec<&str>>() {
        let machine_info: Vec<&str> = machine.lines().collect();
        let (x1, y1) = get_move_amount(machine_info[0]);
        let (x2, y2) = get_move_amount(machine_info[1]);
        let (c1, c2) = get_prize_coords(machine_info[2]);

        let delta = (x1 * y2) - (x2 * y1);

        if delta == 0f64 {
            continue;
        } else {
            let cx = c1 + 10000000000000f64; //just remove this big addition for part 1
            let cy = c2 + 10000000000000f64;

            let delta1: f64 = (cx * y2 - cy * x2);
            let delta2: f64 = (cy * x1 - cx * y1);

            let a = delta1 / delta;
            let b = delta2 / delta;

            // also for part 1, both a and b must be less than 100
            // that condition must also be added here
            if (a.ceil() == a && b.ceil() == b) && (a > 0f64 && b > 0f64) {
                cost += 3f64 * a + b;
            }
        }
    }
    println!("Part 2: {}", cost)
}

fn get_move_amount(button_info: &str) -> (f64, f64) {
    let parsed = button_info.split("+").collect::<Vec<&str>>();
    (
        parsed[1].split(",").collect::<Vec<_>>()[0].parse().unwrap(),
        parsed[2].parse().unwrap(),
    )
}

fn get_prize_coords(button_info: &str) -> (f64, f64) {
    let parsed = button_info.split("=").collect::<Vec<&str>>();
    (
        parsed[1].split(",").collect::<Vec<_>>()[0].parse().unwrap(),
        parsed[2].parse().unwrap(),
    )
}
