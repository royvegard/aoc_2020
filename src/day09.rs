#[aoc(day9, part1)]
pub fn solve_part1(input: &str) -> usize {
    find_first_invalid(input, 25)
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &str) -> usize {
    find_contiguous_sum(input, find_first_invalid(input, 25))
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

fn find_contiguous_sum(input: &str, target: usize) -> usize {
    let data: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();

    let mut first = 0;
    let mut last = 0;

    for i in 0..data.len() {
        let mut length = 1;
        let mut found_sum = false;

        loop {
            let sum = data[i..i + length].iter().sum::<usize>();

            if sum == target {
                first = i;
                last = i + length;
                found_sum = true;
                break;
            } else if sum > target {
                break;
            }

            length += 1;
        }

        if found_sum {
            break;
        }
    }

    data[first..last].iter().max().unwrap() + data[first..last].iter().min().unwrap()
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

    #[test]
    fn find_contiguous() {
        assert!(find_contiguous_sum(EXAMPLE_DATA, 127) == 62);
    }
}
