use aoc_runner_derive::aoc;

struct Slope {
    right: usize,
    down: usize,
}

fn count_trees(input: &str, path: Slope) -> u32 {
    let mut x_position = 0;
    let mut y_position = 0;
    let mut trees_encountered = 0;
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();

    while y_position < lines.len() {
        let position_char: char = lines[y_position].chars().nth(x_position).unwrap();

        x_position = (x_position + path.right) % width;
        y_position += path.down;

        if position_char == '#' {
            trees_encountered += 1;
        }
    }

    trees_encountered
}

fn count_trees_cycle(input: &str, path: Slope) -> u32 {
    let mut x_position = 0;
    let mut y_position = 0;
    let mut trees_encountered = 0;
    let lines: Vec<&str> = input.lines().collect();

    while y_position < lines.len() {
        let position_char: char = lines[y_position].chars().cycle().nth(x_position).unwrap();

        x_position += path.right;
        y_position += path.down;

        if position_char == '#' {
            trees_encountered += 1;
        }
    }

    trees_encountered
}

#[aoc(day3, part1, modulus)]
fn solve_part1(input: &str) -> u32 {
    count_trees(input, Slope { right: 3, down: 1 })
}

#[aoc(day3, part1, cycle)]
fn solve_part1_cycle(input: &str) -> u32 {
    count_trees_cycle(input, Slope { right: 3, down: 1 })
}

#[aoc(day3, part2, modulus)]
fn solve_part2(input: &str) -> u32 {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let mut tree_product = 1;

    for slope in slopes {
        tree_product *= count_trees(input, slope);
    }

    tree_product
}

#[aoc(day3, part2, cycle)]
fn solve_part2_cycle(input: &str) -> u32 {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];
    let mut tree_product = 1;

    for slope in slopes {
        tree_product *= count_trees_cycle(input, slope);
    }

    tree_product
}
