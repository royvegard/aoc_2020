#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    find_first_invalid(input, 25)
}

fn find_first_invalid(input: &str, preamble: usize) -> usize {
    let data: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    let mut front = preamble;

    loop {
        if front >= data.len() {
            return 0;
        } else if !check_2sum(&data[front - preamble..front], data[front]) {
            return data[front];
        }

        front += 1;
    }
}

fn check_2sum(input: &[usize], target: usize) -> bool {
    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if input[i] + input[j] == target && input[i] != input[j] {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA: &str = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn find_invalid() {
        assert!(find_first_invalid(EXAMPLE_DATA, 5) == 127);
    }
}
