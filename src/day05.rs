use aoc_runner_derive::aoc;

#[aoc(day5, part1)]
fn solve_part1(input: &str) -> u32 {
    let boarding_pass = input.lines().collect::<Vec<&str>>();
    let mut highest_seat_id = 0;

    for pass in boarding_pass {
        let id = get_seat_id(pass);
        if id > highest_seat_id {
            highest_seat_id = id;
        }
    }

    highest_seat_id
}

#[aoc(day5, part2)]
fn solve_part2(input: &str) -> u32 {
    let passes = input.lines().collect::<Vec<&str>>();
    let mut ids = vec![0];

    for pass in passes {
        ids.push(get_seat_id(pass));
    }

    ids.sort();

    for index in 1..ids.len() - 2 {
        if ids[index + 1] - ids[index] > 1 {
            return ids[index] + 1;
        }
    }
    0
}

fn get_seat_id(input: &str) -> u32 {
    let mut rs: [u32; 2] = [0, 127]; // row space
    let mut cs: [u32; 2] = [0, 7]; // column space

    for c in input.chars() {
        match c {
            'F' => rs[1] -= ((rs[1] - 1 - rs[0]) / 2) + 1,
            'B' => rs[0] += ((rs[1] - 1 - rs[0]) / 2) + 1,
            'R' => cs[0] += ((cs[1] - 1 - cs[0]) / 2) + 1,
            'L' => cs[1] -= ((cs[1] - 1 - cs[0]) / 2) + 1,
            _ => {}
        }
    }

    rs[0] * 8 + cs[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boarding_passes() {
        assert!(get_seat_id("BFFFBBFRRR") == 567);
        assert!(get_seat_id("FFFBBBFRRR") == 119);
        assert!(get_seat_id("BBFFBBFRLL") == 820);
    }
}
