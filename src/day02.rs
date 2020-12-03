use aoc_runner_derive::aoc;

#[aoc(day2, part1, loops)]
fn solve_part1(input: &str) -> u32 {
    let mut valid_passwords: u32 = 0;
    for line in input.lines() {
        let data: Vec<&str> = line.split(':').collect();
        let policy = data[0];
        let password = data[1].trim();
        let data: Vec<&str> = policy.split('-').collect();
        let minimum = data[0].parse::<u32>().unwrap();
        let data: Vec<&str> = data[1].split(' ').collect();
        let maximum = data[0].parse::<u32>().unwrap();
        let character = data[1];

        let mut counter = 0;
        for c in password.chars() {
            if c.to_string() == character {
                counter += 1;
            }
        }

        if counter >= minimum && counter <= maximum {
            valid_passwords += 1;
        }
    }
    valid_passwords
}

#[aoc(day2, part2, loops)]
fn solve_part2(input: &str) -> u32 {
    let mut valid_passwords: u32 = 0;
    for line in input.lines() {
        let data: Vec<&str> = line.split(':').collect();
        let policy = data[0];
        let password = data[1].trim();
        let data: Vec<&str> = policy.split('-').collect();
        let position_one = data[0].parse::<u32>().unwrap();
        let data: Vec<&str> = data[1].split(' ').collect();
        let position_two = data[0].parse::<u32>().unwrap();
        let character = data[1];

        let mut found: u32 = 0;
        let mut index: u32 = 1;
        for c in password.chars() {
            if c.to_string() == character {
                if index == position_one {
                    found += 1;
                } else if index == position_two {
                    found += 1;
                }
            }
            index += 1;
        }

        if found == 1 {
            valid_passwords += 1;
        }
    }
    valid_passwords
}
