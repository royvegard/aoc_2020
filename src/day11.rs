#[aoc(day11, part1)]
pub fn solve_part1(input: &str) -> usize {
    game_of_seats(input)
}

#[derive(Clone)]
struct Seat {
    state: char,
    next_state: char,
    adjacent: usize,
}

fn game_of_seats(input: &str) -> usize {
    let mut layout: Vec<Vec<Seat>> = input
        .lines()
        .map(|x| {
            x.chars()
                .map(|x| Seat {
                    state: x,
                    next_state: ' ',
                    adjacent: 0,
                })
                .collect()
        })
        .collect();

    loop {
        for row_no in 0..layout.len() {
            for col_no in 0..layout.first().unwrap().len() {
                let adjacent = get_adjacent(&layout, row_no, col_no);
                layout[row_no][col_no].adjacent = adjacent;
                if layout[row_no][col_no].state == 'L' && adjacent == 0 {
                    layout[row_no][col_no].next_state = '#';
                } else if layout[row_no][col_no].state == '#' && adjacent >= 4 {
                    layout[row_no][col_no].next_state = 'L';
                } else if layout[row_no][col_no].state == '.' {
                    layout[row_no][col_no].next_state = '.';
                } else {
                    layout[row_no][col_no].next_state = layout[row_no][col_no].state;
                }
            }
        }

        if is_stable(&layout) {
            break;
        }

        for row_no in 0..layout.len() {
            for col_no in 0..layout.first().unwrap().len() {
                layout[row_no][col_no].state = layout[row_no][col_no].next_state;
            }
        }
    }

    get_occupied(&layout)
}

fn is_stable(layout: &Vec<Vec<Seat>>) -> bool {
    let mut stable = true;
    for row_no in 0..layout.len() {
        for col_no in 0..layout.first().unwrap().len() {
            if !(layout[row_no][col_no].state == layout[row_no][col_no].next_state) {
                stable = false;
            }
        }
    }

    stable
}

fn get_occupied(layout: &Vec<Vec<Seat>>) -> usize {
    let mut occupied = 0;

    for row_no in 0..layout.len() {
        for col_no in 0..layout.first().unwrap().len() {
            if layout[row_no][col_no].state == '#' {
                occupied += 1;
            }
        }
    }

    occupied
}

fn get_adjacent(layout: &Vec<Vec<Seat>>, row: usize, col: usize) -> usize {
    let mut adj = 0;
    let last_row = layout.len() - 1;
    let last_col = layout.first().unwrap().len() - 1;

    for r in &[row as isize - 1, row as isize, row as isize + 1] {
        for c in &[col as isize - 1, col as isize, col as isize + 1] {
            if r == &(row as isize) && c == &(col as isize) {
                continue;
            }
            if r < &0 || c < &0 {
                continue;
            }
            if r > &(last_row as isize) || c > &(last_col as isize) {
                continue;
            }

            if layout[*r as usize][*c as usize].state == '#' {
                adj += 1;
            }
        }
    }

    adj
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_1: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn play_game_of_seats() {
        assert!(game_of_seats(EXAMPLE_DATA_1) == 37);
    }
}
