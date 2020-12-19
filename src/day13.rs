use std::collections::HashMap;

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (usize, Vec<usize>) {
    let mut a = input.lines();
    let start_time = a.next().unwrap().parse::<usize>().unwrap();
    let bus = a
        .next()
        .unwrap()
        .split(',')
        .map(|x| {
            let n = x.parse::<usize>();
            match n {
                Ok(t) => t,
                Err(_) => 0,
            }
        })
        .collect::<Vec<usize>>();

    (start_time, bus)
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &(usize, Vec<usize>)) -> usize {
    let (time, id) = get_earliest_time(input);

    time * id
}

#[aoc(day13, part2)]
pub fn solve_part2(input: &(usize, Vec<usize>)) -> usize {
    get_sequence_start(&input.1)
}

fn get_earliest_time(input: &(usize, Vec<usize>)) -> (usize, usize) {
    let start_time = input.0;
    let bus = &input.1;

    let mut bus_found = 0;
    let mut wait_time = 0;

    while bus_found == 0 {
        wait_time += 1;

        for b in bus {
            if *b == 0 {
                continue;
            }

            if (start_time + wait_time) % *b == 0 {
                bus_found = *b;
                break;
            }
        }
    }

    (wait_time, bus_found)
}

fn get_sequence_start(id: &Vec<usize>) -> usize {
    let mut sorted_id = id.clone();
    sorted_id.sort();
    sorted_id.reverse();
    let mut region_candidate = 0;
    let mut start_candidate;

    let mut bus_ids = HashMap::new();
    for (i, bus) in id.iter().enumerate() {
        if bus == &0 {
            continue;
        }
        bus_ids.insert(bus, i);
    }

    loop {
        let mut found_it = true;
        region_candidate += sorted_id[0];
        assert!(region_candidate % sorted_id[0] == 0);
        print!("Region candidate: {}\r", region_candidate);
        start_candidate = region_candidate - bus_ids[&sorted_id[0]];

        for (i, b) in id.iter().enumerate() {
            if b == &0 {
                continue;
            }
            if (start_candidate + i) % b != 0 {
                found_it = false;
                break;
            }
        }

        if found_it {
            break;
        }
    }

    start_candidate
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA1: &str = "939
7,13,x,x,59,x,31,19";

    #[test]
    fn example_bus_table_early() {
        let (time, id) = parse_input(EXAMPLE_DATA1);
        let (wait_time, bus_id) = get_earliest_time(&(time, id));

        assert!(wait_time * bus_id == 295);
    }

    #[test]
    fn example_1_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA1);

        assert!(get_sequence_start(&id) == 1068781);
    }

    const EXAMPLE_DATA2: &str = "55\n17,x,13,19";
    #[test]
    fn example_2_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA2);

        assert!(get_sequence_start(&id) == 3417);
    }

    const EXAMPLE_DATA3: &str = "55\n67,7,59,61";
    #[test]
    fn example_3_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA3);

        assert!(get_sequence_start(&id) == 754018);
    }

    const EXAMPLE_DATA4: &str = "55\n67,x,7,59,61";
    #[test]
    fn example_4_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA4);

        assert!(get_sequence_start(&id) == 779210);
    }

    const EXAMPLE_DATA5: &str = "55\n67,7,x,59,61";
    #[test]
    fn example_5_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA5);

        assert!(get_sequence_start(&id) == 1261476);
    }

    const EXAMPLE_DATA6: &str = "55\n1789,37,47,1889";
    #[test]
    fn example_6_sequence_start() {
        let (_, id) = parse_input(EXAMPLE_DATA6);

        assert!(get_sequence_start(&id) == 1_202_161_486);
    }
}
