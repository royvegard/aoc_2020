use aoc_runner_derive::aoc;
use std::collections::HashSet;

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> u32 {
    let forms = input.split("\n\n").collect::<Vec<&str>>();
    let mut sum_of_yes = 0;

    for form in forms {
        sum_of_yes += get_yes_answers(form);
    }

    sum_of_yes
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> u32 {
    let forms = input.split("\n\n").collect::<Vec<&str>>();
    let mut sum_of_yes = 0;

    for form in forms {
        sum_of_yes += get_everyone_yes_answers(form);
    }

    sum_of_yes
}

fn get_yes_answers(input: &str) -> u32 {
    let mut yes: HashSet<char> = HashSet::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            yes.insert(c);
        }
    }

    yes.len() as u32
}

fn get_everyone_yes_answers(input: &str) -> u32 {
    let persons = input.lines().collect::<Vec<&str>>();
    let mut yes: HashSet<char> = HashSet::new();

    match persons.split_first() {
        Some((first, others)) => {
            for c in first.chars() {
                if others.len() > 0 {
                    let mut found_in_all_others = true;

                    for other in others {
                        if !other.contains(c) {
                            found_in_all_others = false;
                        }
                    }

                    if found_in_all_others {
                        yes.insert(c);
                    }
                } else {
                    yes.insert(c);
                }
            }
        }
        None => {}
    }

    yes.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_anyone_answer_yes() {
        assert!(get_yes_answers("abc") == 3);
        assert!(get_yes_answers("a\nb\nc") == 3);
        assert!(get_yes_answers("ab\nac") == 3);
        assert!(get_yes_answers("a\na\na\na") == 1);
        assert!(get_yes_answers("b") == 1);
    }

    #[test]
    fn count_everyone_answer_yes() {
        assert!(get_everyone_yes_answers("abc") == 3);
        assert!(get_everyone_yes_answers("a\nb\nc") == 0);
        assert!(get_everyone_yes_answers("ab\nac") == 1);
        assert!(get_everyone_yes_answers("a\na\na\na") == 1);
        assert!(get_everyone_yes_answers("b") == 1);
    }
}
