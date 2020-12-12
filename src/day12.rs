#[aoc(day12, part1)]
pub fn solve_part1(input: &str) -> usize {
    get_distance(input)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &str) -> usize {
    get_distance_2(input)
}

struct Ferry {
    x: isize,
    y: isize,
    direction: isize,
}

fn get_distance(input: &str) -> usize {
    // start at origin 0,0 facing east
    let mut boat = Ferry {
        x: 0,
        y: 0,
        direction: 90,
    };

    let instructions: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.split_at(1))
        .collect::<Vec<(&str, &str)>>();

    for i in instructions {
        let value = i.1.parse::<isize>().unwrap();

        match i.0 {
            "N" => boat.y += value,
            "S" => boat.y -= value,
            "E" => boat.x += value,
            "W" => boat.x -= value,
            "L" => boat.direction = boat.direction - value,
            "R" => boat.direction = boat.direction + value,
            "F" => match boat.direction {
                0 => boat.y += value,
                180 => boat.y -= value,
                90 => boat.x += value,
                270 => boat.x -= value,
                _ => {}
            },
            _ => {}
        }
        if boat.direction >= 360 {
            boat.direction -= 360;
        }
        if boat.direction < 0 {
            boat.direction += 360;
        }
    }

    (isize::abs(boat.x) + isize::abs(boat.y)) as usize
}

fn cos(phi: isize) -> isize {
    let mut angle = phi;

    if angle >= 360 {
        angle -= 360;
    }
    if angle < 0 {
        angle += 360
    }

    match angle {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => 0,
    }
}

fn sin(phi: isize) -> isize {
    let mut angle = phi;

    if angle >= 360 {
        angle -= 360;
    }
    if angle < 0 {
        angle += 360
    }

    match angle {
        0 => 0,
        90 => 1,
        180 => 0,
        270 => -1,
        _ => 0,
    }
}

fn get_distance_2(input: &str) -> usize {
    // start at origin 0,0 facing east
    let mut boat = Ferry {
        x: 0,
        y: 0,
        direction: 90,
    };

    let mut wp = Ferry {
        x: 10,
        y: 1,
        direction: 0,
    };

    let instructions: Vec<(&str, &str)> = input
        .lines()
        .map(|x| x.split_at(1))
        .collect::<Vec<(&str, &str)>>();

    for i in instructions {
        let value = i.1.parse::<isize>().unwrap();

        match i.0 {
            "N" => wp.y += value,
            "S" => wp.y -= value,
            "E" => wp.x += value,
            "W" => wp.x -= value,
            "L" => {
                let new_x = wp.x * cos(value) - wp.y * sin(value);
                let new_y = wp.x * sin(value) + wp.y * cos(value);
                wp.x = new_x;
                wp.y = new_y;
            }
            "R" => {
                let new_x = wp.x * cos(value) + wp.y * sin(value);
                let new_y = wp.y * cos(value) - wp.x * sin(value);
                wp.x = new_x;
                wp.y = new_y;
            }
            "F" => {
                boat.x += wp.x * value;
                boat.y += wp.y * value;
            }
            _ => {}
        }
    }

    (isize::abs(boat.x) + isize::abs(boat.y)) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "F10
N3
F7
R90
F11
";

    #[test]
    fn example_distance() {
        assert!(get_distance(EXAMPLE_DATA) == 25);
    }

    #[test]
    fn example_distance2() {
        assert!(get_distance_2(EXAMPLE_DATA) == 286);
    }
}
