#[aoc(day10, part1)]
pub fn solve_part1(input: &str) -> usize {
    find_jolts(input)
}

fn find_jolts(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    // Add outlet and device
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    let mut jolt_1 = 0;
    let mut jolt_3 = 0;

    for i in 1..adapters.len() {
        if adapters[i] - adapters[i - 1] == 1 {
            jolt_1 += 1;
        } else if adapters[i] - adapters[i - 1] == 3 {
            jolt_3 += 1;
        }
    }
    jolt_1 * jolt_3
}

fn find_arrangements(input: &str) -> usize {
    let mut adapters: Vec<usize> = input.lines().map(|x| x.parse().unwrap()).collect();
    // Add outlet and device
    adapters.push(0);
    adapters.push(adapters.iter().max().unwrap() + 3);
    adapters.sort();

    let mut paths = vec![0; adapters.len()];
    paths[0] = 0;
    paths[1] = 0;
    paths[2] = 1;

    for i in 3..adapters.len() {
        for j in i - 3..i {
            if adapters[i] - adapters[j] <= 3 {
                paths[i] += paths[j];
            }
        }
    }

    paths[paths.len() - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_DATA_1: &str = "16
10
15
5
1
11
7
19
6
12
4";

    const EXAMPLE_DATA_2: &str = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";

    #[test]
    fn find_jolt_product() {
        assert!(find_jolts(EXAMPLE_DATA_1) == 7 * 5);
        assert!(find_jolts(EXAMPLE_DATA_2) == 22 * 10);
    }

    #[test]
    fn test_find_arrangements() {
        //assert!(find_arrangements(EXAMPLE_DATA_1) == 8);
        //assert!(find_arrangements(EXAMPLE_DATA_2) == 19208);
    }
}
