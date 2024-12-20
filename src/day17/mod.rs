// TODO: PART 2 not done yet

pub fn day17_part1(content: &str) {
    // our registers
    let mut A: u32 = content
        .trim()
        .lines()
        .filter(|&line| line.contains("A"))
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let B: u32 = content
        .trim()
        .lines()
        .filter(|&line| line.contains("B"))
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let C: u32 = content
        .trim()
        .lines()
        .filter(|&line| line.contains("C"))
        .nth(0)
        .unwrap()
        .split(":")
        .nth(1)
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let program_string = content
        .trim()
        .lines()
        .filter(|&line| line.contains("Program"))
        .nth(0)
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .trim();

    println!("Part 1: {}", simulate_program(program_string, A, B, C));
    let mut new_A = A + 1;

    /*
    while new_A < u32::MAX/2 {
        if simulate_program(program_string, new_A, 0, 0) == program_string {
            break;
        }
        println!("{}", new_A);
        new_A += 1;
    }

     */

    println!("Part 2: {}", new_A);
}

fn simulate_program(program_string: &str, A: u32, B: u32, C: u32) -> String {
    let mut program: Vec<char> = program_string.chars().filter(|&c| c != ',').collect();

    let mut A = A;
    let mut B = B;
    let mut C = C;

    let mut ins_pointer = 0;
    let mut should_increment = true;
    let mut output = Vec::new();

    while ins_pointer < program.len() {
        should_increment = true;
        let operand: u32 = program[ins_pointer + 1].to_digit(10).unwrap() as u32;
        match program[ins_pointer] {
            '0' => {
                A = A / 2u32.pow(get_combo_operand(operand, A, B, C) as u32);
            }
            '1' => {
                B = B ^ operand;
            }
            '2' => {
                B = get_combo_operand(operand, A, B, C) % 8;
            }
            '3' => {
                if A != 0 {
                    ins_pointer = operand as usize; // to negate the increment by 2
                    should_increment = false;
                }
            }
            '4' => {
                B = B ^ C;
            }
            '5' => {
                output.push(get_combo_operand(operand, A, B, C) % 8);
            }
            '6' => {
                B = A / 2u32.pow(get_combo_operand(operand, A, B, C) as u32);
            }
            '7' => {
                C = A / 2u32.pow(get_combo_operand(operand, A, B, C) as u32);
            }
            _ => {}
        }

        if should_increment {
            ins_pointer += 2
        }
    }

    let str_out = output
        .iter()
        .map(|&i| i.to_string())
        .collect::<Vec<String>>();

    str_out.join(",")
}

fn get_combo_operand(operand: u32, A: u32, B: u32, C: u32) -> u32 {
    match operand {
        0..=3 => operand,
        4 => A,
        5 => B,
        6 => C,
        _ => u32::MAX,
    }
}
